-- Your SQL goes here

CREATE TABLE `dkp_merchant_channel` (
  `acid` int(11) NOT NULL AUTO_INCREMENT,
  `app_id` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '应用ID',
  `merchant_id` varchar(48) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '应用层级-商户ID',
  `channel_type` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT 'WEPAY' COMMENT '渠道类型：ChannelTypeEnum',
  `channel_rate` double(16,4) DEFAULT '0.0060' COMMENT '渠道终审费率',
  `mer_auth_token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '支付宝授权token',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `is_facepay` int(1) DEFAULT '0' COMMENT '微信刷脸支持',
  `remark` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '备注：临时存放alipay tokens',
  `status` int(1) DEFAULT '0' COMMENT '状态：0关闭，1开启',
  PRIMARY KEY (`acid`) USING BTREE,
  UNIQUE KEY `udx_mch` (`app_id`,`merchant_id`,`channel_type`)
) ENGINE=InnoDB AUTO_INCREMENT=32 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC COMMENT='应用渠道表';