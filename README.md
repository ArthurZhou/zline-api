# zline-api

为zline++编写的api，可部署至Cloudflare Workers

## 开发

`npx wrangler dev` 启动本地开发服务器

`npx wrangler deploy` 部署至Cloudflare

## 作用

将`jincai.sh.cn`原站点上杂乱的数据整理后以RESTful形式发回客户端

## API

统一响应格式:

```json
{
    "code": 200,  // 该值会根据原站点返回的响应码而变化
    "message": "状态消息",
    "data": {
        "key": "value"  // api返回的数据
    }
}
```

### 获取XToken

URL: `/xtoken`

返回数据: `xtoken`: string

#### 示例

成功时响应的json:

```json
{
    "code": 200,
    "message": "ok",
    "data": {
        "xtoken": "<xtoken>"
    }
}
```
