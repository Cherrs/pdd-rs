use std::collections::HashMap;

use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use tinytemplate::{format_unescaped, TinyTemplate};

#[tokio::main]
async fn main() -> Result<()> {
    let mut mods = Mods { mods: Vec::new() };
    let client = reqwest::Client::new();
    let pdd_cat = client
        .post("https://open-api.pinduoduo.com/pop/doc/category/list")
        .send()
        .await?
        .json::<Value>()
        .await?;
    let pdd_cat_result = pdd_cat["result"].as_array().unwrap();

    for i in pdd_cat_result {
        let id = i["id"].as_i64().unwrap();
        let pdd_bycat = client
            .post("https://open-api.pinduoduo.com/pop/doc/info/list/byCat")
            .json(&json!({ "id": id }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        let pdd_reqs = pdd_bycat["result"]["docList"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f["id"].as_str().unwrap());
        for i in pdd_reqs {
            let pdd_req = client
                .post("https://open-api.pinduoduo.com/pop/doc/info/get")
                .json(&json!({ "id": i }))
                .send()
                .await?
                .json::<Value>()
                .await?;
            mods.mods.push(build_template(pdd_req).unwrap());
        }
    }

    let mut tt = TinyTemplate::new();
    let template = std::fs::read("build_req/src/mod")?;
    let template_str = &String::from_utf8(template)?;
    tt.add_template("mod", template_str).unwrap();
    let t = tt.render("mod", &mods).unwrap();
    std::fs::write("src/requests/mod.rs", t).unwrap();
    Ok(())
}

fn build_template(value: Value) -> Result<ModStructs> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    let template = std::fs::read("build_req/src/template")?;
    let template_str = &String::from_utf8(template)?;
    tt.add_template("req", template_str).unwrap();

    let fields = value["result"]["requestParamList"]
        .as_array()
        .unwrap()
        .clone()
        .into_iter()
        .map(|f| serde_json::from_value::<DataReqField>(f).unwrap());

    let name = value["result"]["scopeName"].as_str().unwrap();
    let name = name.to_upper_camel_case();
    let usage_scenarios = value["result"]["usageScenarios"].as_str().unwrap();
    let mut map = HashMap::new();

    let mut structs = HashMap::new();
    for i in fields {
        let mut i = i;
        i.param_name = i.param_raw_name.replace('$', "").to_snake_case();
        if i.param_name == "type" {
            i.param_name = "type_".to_string();
        }

        // 处理特殊字段类型
        if i.param_name == "p_id_list" && name == "PddDdkOauthCashgiftCreate" {
            i.param_type = "Vec<String>".to_string();
        }

        if i.children_num != 0 {
            let mut name_ = i.param_name.to_upper_camel_case();

            //处理特殊字段名
            if name_ == "Request" {
                name_ = format!("{}{}", name, name_);
            }

            if i.param_type == "OBJECT[]" {
                i.param_type = format!("Vec<{}>", name_.clone());
            } else {
                i.param_type = name_.clone();
            }

            map.insert(i.id, name_);
            let v = structs.entry(i.parent_id).or_insert(Vec::new());
            v.push(i);
        } else {
            let v = structs.entry(i.parent_id).or_insert(Vec::new());
            v.push(i);
        }
    }

    let data_req_struct = structs.into_iter().map(|f| {
        if f.0 == 0 {
            DataReqStruct {
                fields: f.1,
                parent_id: f.0,
                struct_desc: usage_scenarios.replace(['\n', '\t'], ""),
                struct_name: name.to_string(),
            }
        } else {
            DataReqStruct {
                fields: f.1,
                parent_id: f.0,
                struct_desc: usage_scenarios.replace(['\n', '\t'], ""),
                struct_name: map[&f.0].clone(),
            }
        }
    });

    let mut tmpdata = TmpData {
        api_name: value["result"]["apiName"].as_str().unwrap().to_string(),
        scope_name: value["result"]["scopeName"].as_str().unwrap().to_string(),
        usage_scenarios: usage_scenarios.to_string(),
        structs: data_req_struct.collect(),
        name: name.clone(),
        response_name: value["result"]["responseParamList"].as_array().unwrap()[0]["paramName"]
            .as_str()
            .unwrap()
            .to_string(),
    };

    if tmpdata.structs.is_empty() {
        tmpdata.structs = vec![DataReqStruct {
            struct_name: name.clone(),
            struct_desc: value["result"]["apiName"].as_str().unwrap().to_string(),
            fields: Vec::new(),
            parent_id: 0,
        }]
    }

    let t = tt.render("req", &tmpdata).unwrap();
    let path = format!(
        "src/requests/{}.rs",
        value["result"]["scopeName"]
            .as_str()
            .unwrap()
            .replace('.', "_")
    );
    std::fs::write(path, t).unwrap();
    Ok(ModStructs {
        file_name: value["result"]["scopeName"]
            .as_str()
            .unwrap()
            .replace('.', "_"),
        struct_name: name,
    })
}

fn get_type<'a, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'a>,
{
    let s = String::deserialize(d)?;
    match s.as_str() {
        "LONG" => Ok("i64".to_string()),
        "INTEGER" => Ok("i32".to_string()),
        "BOOLEAN" => Ok("bool".to_string()),
        "STRING" => Ok("String".to_string()),
        "LONG[]" => Ok("Vec<i64>".to_string()),
        "INTEGER[]" => Ok("Vec<i32>".to_string()),
        "STRING[]" => Ok("Vec<String>".to_string()),
        "FILE" => Ok("PddFile".to_string()),
        "DOUBLE" => Ok("f32".to_string()),
        _ => Ok(s),
    }
}

fn rep<'a, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'a>,
{
    let s = String::deserialize(d)?;
    Ok(s.replace(['\n', '\t'], ""))
}

#[derive(Serialize)]
struct Mods {
    mods: Vec<ModStructs>,
}

#[derive(Serialize)]
struct ModStructs {
    file_name: String,
    struct_name: String,
}

#[derive(Serialize)]
struct TmpData {
    api_name: String,
    scope_name: String,
    usage_scenarios: String,
    name: String,
    structs: Vec<DataReqStruct>,
    response_name: String,
}

#[derive(Serialize, Deserialize)]
struct DataReqStruct {
    parent_id: i64,
    struct_name: String,
    struct_desc: String,
    fields: Vec<DataReqField>,
}

#[derive(Serialize, Deserialize)]
struct DataReqField {
    id: i64,
    #[serde(alias = "parentId")]
    parent_id: i64,
    #[serde(alias = "childrenNum")]
    children_num: i64,
    #[serde(alias = "paramName")]
    param_raw_name: String,
    #[serde(alias = "paramType")]
    #[serde(deserialize_with = "get_type")]
    param_type: String,
    #[serde(alias = "paramDesc")]
    #[serde(deserialize_with = "rep")]
    param_desc: String,
    #[serde(default)]
    param_name: String,
}
