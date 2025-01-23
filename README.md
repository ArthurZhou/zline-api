# zline-api
为zline++编写的api，可部署至Cloudflare Workers

## 开发
`npx wrangler dev` 启动本地开发服务器

`npx wrangler deploy` 部署至Cloudflare

## 作用
将`jincai.sh.cn`原站点上杂乱的数据整理后以RESTful形式发回客户端


## API
除特殊标注外，API统一使用`GET`，参数使用URL params传递

传参格式:
`https://<host>/url?param1=blablabla&param2=...`

响应格式:
```json
{
    "code": 200,  // 该值会根据原站点返回的响应码而变化
    "message": "状态消息",
    "data": {
        "key": "value"  // api返回的数据
    }
}
```
***

### 获取XToken
URL: `/security/xtoken`

返回数据: 
- `xtoken`: string = `XXXX_pzXXXXXX`

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
***

### 登录
URL: `/security/login`

传入参数: 
- `xtoken`: 加密后的客户端xtoken
- `username`: 加密后的用户名
- `password`: 加密后的密码

返回数据: 
- `cookie`: cookie值 `PZLSystemLogin=XXXXXXXX;PHPSESSID=XXXX`
    
    > `PZLSystemLogin`是侧栏等控件的cookie
    
    > `PHPSESSID`是嵌入的iframe的cookie
***

### 退出登录
URL: `/security/logout`

**注意: 本API仅向服务端发送请求，不返回用于清除cookie的headers，调用后需手动清除客户端cookie和登录状态**
***

### 检验登录状态
**目前该功能不检查嵌入PHP页面的cookie**

URL: `/security/status`

传入参数: 
- `cookie`: 需要检验的cookie

返回数据: 
- `valid`: bool true/false
***

### 获取考试场次和科目

URL: `/data/exam_list`

传入参数: 
- `cookie`

返回数据: 
- `subject`: 科目的`"<名称>": "<对应代码>"`
- `time`: 场次的`"<名称>": "<对应代码>"`
***