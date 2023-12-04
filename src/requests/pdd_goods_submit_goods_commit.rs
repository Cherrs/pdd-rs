use crate::Request;

use serde::{Deserialize, Serialize};


/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ElecGoodsAttributes {
    
    /// 开始时间（timeType=1时必填表示核销的开始时间）（精确到毫秒）
    #[serde(rename = "begin_time")]
    pub begin_time: Option<i64>,
    
    /// 天数内有效（timeType=3必填，表示发货后几天内核销）
    #[serde(rename = "days_time")]
    pub days_time: Option<i32>,
    
    /// 截止时间（timeType=1,2时必填，表示发货后核销的截止时间）（精确到毫秒）
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
    
    /// 卡券核销类型（1：起始时间内有效，2：发货后后至截止时间内有效，3：发货后多少天内有效）
    #[serde(rename = "time_type")]
    pub time_type: Option<i32>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SkuList {
    
    /// sku上架状态，0-已下架，1-上架中
    #[serde(rename = "is_onsale")]
    pub is_onsale: Option<i32>,
    
    /// sku送装参数：长度
    #[serde(rename = "length")]
    pub length: Option<i64>,
    
    /// sku购买限制，只入参999
    #[serde(rename = "limit_quantity")]
    pub limit_quantity: Option<i64>,
    
    /// 商品团购价格
    #[serde(rename = "multi_price")]
    pub multi_price: Option<i64>,
    
    /// 商品sku外部编码
    #[serde(rename = "out_sku_sn")]
    pub out_sku_sn: Option<String>,
    
    /// 第三方sku Id
    #[serde(rename = "out_source_sku_id")]
    pub out_source_sku_id: Option<String>,
    
    /// oversea_sku
    #[serde(rename = "oversea_sku")]
    pub oversea_sku: Option<OverseaSku>,
    
    /// 商品单买价格
    #[serde(rename = "price")]
    pub price: Option<i64>,
    
    /// 商品sku库存初始数量，后续库存update只使用stocks.update接口进行调用
    #[serde(rename = "quantity")]
    pub quantity: Option<i64>,
    
    /// sku预售时间戳，单位秒；不更新传null，取消传0，更新传实际值
    #[serde(rename = "sku_pre_sale_time")]
    pub sku_pre_sale_time: Option<i32>,
    
    /// sku属性
    #[serde(rename = "sku_properties")]
    pub sku_properties: Option<Vec<SkuProperties>>,
    
    /// 商品规格列表，根据pdd.goods.spec.id.get生成的规格属性id，例如：颜色规格下商家新增白色和黑色，大小规格下商家新增L和XL，则由4种spec组合，入参一种组合即可，在skulist中需要有4个spec组合的sku
    #[serde(rename = "spec_id_list")]
    pub spec_id_list: Option<Vec<i64>>,
    
    /// sku预览图，预览图尺寸：等宽高，且高度不低于480px，现已支持1M大小，越清晰越好卖，SKU预览图格式：仅支持JPG,PNG格式
    #[serde(rename = "thumb_url")]
    pub thumb_url: Option<String>,
    
    /// 重量，单位为g
    #[serde(rename = "weight")]
    pub weight: Option<i64>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoodsTradeAttr {
    
    /// 提前预定天数，默认为0表示当天可预定
    #[serde(rename = "advances_days")]
    pub advances_days: Option<i32>,
    
    /// 预订须知
    #[serde(rename = "booking_notes")]
    pub booking_notes: Option<BookingNotes>,
    
    /// 卡券有效期，日历日期后多少天可用。默认值为0表示仅限日历日当天使用
    #[serde(rename = "life_span")]
    pub life_span: Option<i32>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SkuProperties {
    
    /// 属性单位
    #[serde(rename = "punit")]
    pub punit: Option<String>,
    
    /// 属性id
    #[serde(rename = "ref_pid")]
    pub ref_pid: Option<i64>,
    
    /// 属性值
    #[serde(rename = "value")]
    pub value: Option<String>,
    
    /// 属性值id
    #[serde(rename = "vid")]
    pub vid: Option<i64>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoodsProperties {
    
    /// 组id，非销售属性不用传
    #[serde(rename = "group_id")]
    pub group_id: Option<i32>,
    
    /// 图片url，非销售属性不用传
    #[serde(rename = "img_url")]
    pub img_url: Option<String>,
    
    /// 备注，非销售属性不用传
    #[serde(rename = "note")]
    pub note: Option<String>,
    
    /// 父属性id，非销售属性不用传
    #[serde(rename = "parent_spec_id")]
    pub parent_spec_id: Option<i64>,
    
    /// ref_pid
    #[serde(rename = "ref_pid")]
    pub ref_pid: Option<i64>,
    
    /// 属性id，非销售属性不用传
    #[serde(rename = "spec_id")]
    pub spec_id: Option<i64>,
    
    /// 模板属性id
    #[serde(rename = "template_pid")]
    pub template_pid: Option<i64>,
    
    /// 属性值
    #[serde(rename = "value")]
    pub value: Option<String>,
    
    /// 属性单位
    #[serde(rename = "value_unit")]
    pub value_unit: Option<String>,
    
    /// 属性值id
    #[serde(rename = "vid")]
    pub vid: Option<i64>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OverseaGoods {
    
    /// 保税仓唯一标识
    #[serde(rename = "bonded_warehouse_key")]
    pub bonded_warehouse_key: Option<String>,
    
    /// 消费税率
    #[serde(rename = "consumption_tax_rate")]
    pub consumption_tax_rate: Option<i32>,
    
    /// 清关服务商
    #[serde(rename = "customs_broker")]
    pub customs_broker: Option<String>,
    
    /// 海关编号
    #[serde(rename = "hs_code")]
    pub hs_code: Option<String>,
    
    /// 增值税率
    #[serde(rename = "value_added_tax_rate")]
    pub value_added_tax_rate: Option<i32>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BookingNotes {
    
    /// 预定须知图片地址
    #[serde(rename = "url")]
    pub url: Option<String>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OverseaSku {
    
    /// 计量单位编码，从接口pdd.gooods.sku.measurement.list获取code
    #[serde(rename = "measurement_code")]
    pub measurement_code: Option<String>,
    
    /// 规格
    #[serde(rename = "specifications")]
    pub specifications: Option<String>,
    
    /// 税费
    #[serde(rename = "taxation")]
    pub taxation: Option<i32>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PddGoodsSubmitGoodsCommit {
    
    /// 是否自动补充标品属性
    #[serde(rename = "auto_fill_spu_property")]
    pub auto_fill_spu_property: Option<bool>,
    
    /// 坏果包赔
    #[serde(rename = "bad_fruit_claim")]
    pub bad_fruit_claim: Option<i32>,
    
    /// 限购次数
    #[serde(rename = "buy_limit")]
    pub buy_limit: Option<i64>,
    
    /// 商品轮播图，按次序上传，图片格式支持JPEG/JPG/PNG， 图片尺寸长宽比1：1且尺寸不低于480px，图片大小最高1MB
    #[serde(rename = "carousel_gallery")]
    pub carousel_gallery: Option<Vec<String>>,
    
    /// 商品视频
    #[serde(rename = "carousel_video")]
    pub carousel_video: Option<Vec<CarouselVideo>>,
    
    /// 轮播视频字段
    #[serde(rename = "carousel_video_url")]
    pub carousel_video_url: Option<String>,
    
    /// 叶子类目ID
    #[serde(rename = "cat_id")]
    pub cat_id: Option<i64>,
    
    /// 物流运费模板ID，可使用pdd.logistics.template.get获取
    #[serde(rename = "cost_template_id")]
    pub cost_template_id: Option<i64>,
    
    /// 地区/国家ID，0-中国，暂时只传0（普通商品）
    #[serde(rename = "country_id")]
    pub country_id: Option<i32>,
    
    /// 团购人数
    #[serde(rename = "customer_num")]
    pub customer_num: Option<i64>,
    
    /// 海关名称，只在goods_type为直供商品时有效（现阶段暂不支持）
    #[serde(rename = "customs")]
    pub customs: Option<String>,
    
    /// 是否当日发货,0 否，1 是
    #[serde(rename = "delivery_one_day")]
    pub delivery_one_day: Option<i32>,
    
    /// 发货方式。0：无物流发货；1：有物流发货。
    #[serde(rename = "delivery_type")]
    pub delivery_type: Option<i32>,
    
    /// 商品详情图：a. 尺寸要求宽度处于480~1200px之间，高度0-1500px之间b. 大小1M以内c. 数量限制在20张之间d. 图片格式仅支持JPG,PNG格式e. 点击上传时，支持批量上传详情图
    #[serde(rename = "detail_gallery")]
    pub detail_gallery: Option<Vec<String>>,
    
    /// 卡券类商品属性
    #[serde(rename = "elec_goods_attributes")]
    pub elec_goods_attributes: Option<ElecGoodsAttributes>,
    
    /// 草稿id
    #[serde(rename = "goods_commit_id")]
    pub goods_commit_id: Option<i64>,
    
    /// 商品描述， 字数限制：20-500，例如，新包装，保证产品的口感和新鲜度。单颗独立小包装，双重营养，1斤家庭分享装，更实惠新疆一级骏枣夹核桃仁。
    #[serde(rename = "goods_desc")]
    pub goods_desc: Option<String>,
    
    /// 1213414
    #[serde(rename = "goods_id")]
    pub goods_id: Option<i64>,
    
    /// 商品标题，例如，新疆特产 红满疆枣夹核桃500g
    #[serde(rename = "goods_name")]
    pub goods_name: Option<String>,
    
    /// 商品属性列表
    #[serde(rename = "goods_properties")]
    pub goods_properties: Option<Vec<GoodsProperties>>,
    
    /// 日历商品交易相关信息
    #[serde(rename = "goods_trade_attr")]
    pub goods_trade_attr: Option<GoodsTradeAttr>,
    
    /// 商品出行信息
    #[serde(rename = "goods_travel_attr")]
    pub goods_travel_attr: Option<GoodsTravelAttr>,
    
    /// 1-国内普通商品，2-一般贸易，3-保税仓BBC直供，4-海外BC直邮 ,5-流量 ,6-话费 ,7-优惠券 ,8-QQ充值 ,9-加油卡，15-商家卡券，18-海外CC行邮 19-平台卡券
    #[serde(rename = "goods_type")]
    pub goods_type: Option<i32>,
    
    /// 是否获取商品发布警告信息，默认为否
    #[serde(rename = "ignore_edit_warn")]
    pub ignore_edit_warn: Option<bool>,
    
    /// 商品主图，请参考拼多多首页大图，如果商品参加部分活动则必填，否则无法参加活动a. 尺寸750 x 352pxb. 大小100k以内c. 图片格式仅支持JPG,PNG格式d. 图片背景应以纯白为主, 商品图案居中显示e. 图片不可以添加任何品牌相关文字或logo
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    
    /// 是否支持正品发票
    #[serde(rename = "invoice_status")]
    pub invoice_status: Option<i32>,
    
    /// 是否需要上报海关，现阶段入参默认false，入参true会失败
    #[serde(rename = "is_customs")]
    pub is_customs: Option<bool>,
    
    /// 是否支持假一赔十，false-不支持，true-支持
    #[serde(rename = "is_folt")]
    pub is_folt: Option<bool>,
    
    /// 是否成团预售
    #[serde(rename = "is_group_pre_sale")]
    pub is_group_pre_sale: Option<i32>,
    
    /// 是否预售,true-预售商品，false-非预售商品
    #[serde(rename = "is_pre_sale")]
    pub is_pre_sale: Option<bool>,
    
    /// 是否7天无理由退换货，true-支持，false-不支持
    #[serde(rename = "is_refundable")]
    pub is_refundable: Option<bool>,
    
    /// 是否sku预售，1：是，0：否
    #[serde(rename = "is_sku_pre_sale")]
    pub is_sku_pre_sale: Option<i32>,
    
    /// 缺重包退
    #[serde(rename = "lack_of_weight_claim")]
    pub lack_of_weight_claim: Option<i32>,
    
    /// 本地服务id
    #[serde(rename = "local_service_id_list")]
    pub local_service_id_list: Option<Vec<i32>>,
    
    /// 买家自提模版id
    #[serde(rename = "mai_jia_zi_ti")]
    pub mai_jia_zi_ti: Option<String>,
    
    /// 参考价格，单位为分
    #[serde(rename = "market_price")]
    pub market_price: Option<i64>,
    
    /// 0:提交， 1：保存（默认提交）
    #[serde(rename = "operate_type")]
    pub operate_type: Option<i32>,
    
    /// 单次限量
    #[serde(rename = "order_limit")]
    pub order_limit: Option<i64>,
    
    /// 原产地id，是指海淘商品的生产地址，仅在goods type=3/4的时候必填，可以通过pdd.goods.country.get获取
    #[serde(rename = "origin_country_id")]
    pub origin_country_id: Option<i32>,
    
    /// 商品goods外部编码
    #[serde(rename = "out_goods_id")]
    pub out_goods_id: Option<String>,
    
    /// 第三方商品Id
    #[serde(rename = "out_source_goods_id")]
    pub out_source_goods_id: Option<String>,
    
    /// 第三方商品来源
    #[serde(rename = "out_source_type")]
    pub out_source_type: Option<i32>,
    
    /// {"consumption_tax_rate": 1,"value_added_tax_rate": 9,"hs_code": "2200","customs_broker": "sss","customs_declaration_method": 1,"bonded_warehouse": "sss","bonded_warehouse_key": "pp"}
    #[serde(rename = "oversea_goods")]
    pub oversea_goods: Option<OverseaGoods>,
    
    /// oversea_type
    #[serde(rename = "oversea_type")]
    pub oversea_type: Option<i32>,
    
    /// 预售时间，is_pre_sale为1时必传，UNIX时间戳
    #[serde(rename = "pre_sale_time")]
    pub pre_sale_time: Option<i64>,
    
    /// 0：不支持全国联保；1：支持全国联保
    #[serde(rename = "quan_guo_lian_bao")]
    pub quan_guo_lian_bao: Option<i32>,
    
    /// 是否二手商品，true -二手商品 ，false-全新商品
    #[serde(rename = "second_hand")]
    pub second_hand: Option<bool>,
    
    /// 上门安装模版id
    #[serde(rename = "shang_men_an_zhuang")]
    pub shang_men_an_zhuang: Option<String>,
    
    /// 承诺发货时间（ 秒），48小时或24小时，is_pre_sale为1时不必传
    #[serde(rename = "shipment_limit_second")]
    pub shipment_limit_second: Option<i64>,
    
    /// 门店组id
    #[serde(rename = "shop_group_id")]
    pub shop_group_id: Option<i64>,
    
    /// sku对象列表,实例：[{"is_onsale": 1,"limit_quantity": 999,"price": "2200","weight": 1000,"multi_price": "1900","thumb_url": "http://t06img.yangkeduo.com/images/2018-04-15/ced035033b5d40b589140af882621c03.jpg","out_sku_sn": "L","quantity": 100,"spec_id_list": "[25]","oversea_sku": {"measurement_code": "计量单位编码","taxation": "税费","specifications": "规格"}}]
    #[serde(rename = "sku_list")]
    pub sku_list: Option<Vec<SkuList>>,
    
    /// 库存方式（0：普通型，1：日历型）
    #[serde(rename = "sku_type")]
    pub sku_type: Option<i32>,
    
    /// 送货入户并安装模版id
    #[serde(rename = "song_huo_an_zhuang")]
    pub song_huo_an_zhuang: Option<String>,
    
    /// 送货入户模版id
    #[serde(rename = "song_huo_ru_hu")]
    pub song_huo_ru_hu: Option<String>,
    
    /// 提交后上架状态，0:上架,1:保持原样
    #[serde(rename = "sync_goods_operate")]
    pub sync_goods_operate: Option<i32>,
    
    /// 短标题，示例:新包装，保证产品的口感和新鲜度。单颗独立小包装，双重营养，1斤家庭分享装，更实惠新疆一级骏枣夹核桃仁。
    #[serde(rename = "tiny_name")]
    pub tiny_name: Option<String>,
    
    /// 满2件折扣，可选范围0-100, 0表示取消，95表示95折，设置需先查询规则接口获取实际可填范围
    #[serde(rename = "two_pieces_discount")]
    pub two_pieces_discount: Option<i32>,
    
    /// 保税仓，只在goods_type为直供商品时有效（现阶段暂不支持）
    #[serde(rename = "warehouse")]
    pub warehouse: Option<String>,
    
    /// 水果类目温馨提示，只在水果类目商品才生效， 字数限制：商品描述goods_desc+温馨提示总计不超过500字。
    #[serde(rename = "warm_tips")]
    pub warm_tips: Option<String>,
    
    /// 只换不修的天数，目前只支持0和365
    #[serde(rename = "zhi_huan_bu_xiu")]
    pub zhi_huan_bu_xiu: Option<i32>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CarouselVideo {
    
    /// 商品视频id
    #[serde(rename = "file_id")]
    pub file_id: Option<i64>,
    
    /// 商品视频url
    #[serde(rename = "video_url")]
    pub video_url: Option<String>,
    
}

/// 被驳回的商品从草稿提交
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoodsTravelAttr {
    
    /// 出行人是否必填（默认是）
    #[serde(rename = "need_tourist")]
    pub need_tourist: Option<bool>,
    
    /// 日历商品类型1:旅行类,2:住宿类,3:票务类
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    
}


/// 被驳回的商品从草稿提交
impl Request for PddGoodsSubmitGoodsCommit {
    fn get_type() -> String {
        "pdd.goods.submit.goods.commit".to_string()
    }

    fn get_response_name() -> String {
        "goods_update_response".to_string()
    }
}
