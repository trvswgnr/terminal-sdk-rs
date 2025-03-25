# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique object identifier. The format and length of IDs may change over time. | 
**product_variant_id** | **String** | ID of the product variant being subscribed to. | 
**quantity** | **i32** | Quantity of the subscription. | 
**address_id** | **String** | ID of the shipping address used for the subscription. | 
**card_id** | **String** | ID of the card used for the subscription. | 
**schedule** | Option<[**models::SubscriptionSchedule**](Subscription_schedule.md)> |  | [optional]
**next** | Option<**String**> | Next shipment and billing date for the subscription. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


