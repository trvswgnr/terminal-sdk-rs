# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique object identifier. The format and length of IDs may change over time. | 
**index** | Option<**i32**> | Zero-based index of the order for this user only. | [optional]
**shipping** | [**models::OrderShipping**](Order_shipping.md) |  | 
**amount** | [**models::OrderAmount**](Order_amount.md) |  | 
**tracking** | [**models::OrderTracking**](Order_tracking.md) |  | 
**items** | [**Vec<models::OrderItem>**](OrderItem.md) | Items in the order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


