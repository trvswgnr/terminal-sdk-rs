# Product

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique object identifier. The format and length of IDs may change over time. | 
**name** | **String** | Name of the product. | 
**description** | **String** | Description of the product. | 
**variants** | [**Vec<models::ProductVariant>**](ProductVariant.md) | List of variants of the product. | 
**order** | Option<**i32**> | Order of the product used when displaying a sorted list of products. | [optional]
**subscription** | Option<**String**> | Whether the product must be or can be subscribed to. | [optional]
**tags** | Option<[**models::ProductTags**](Product_tags.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


