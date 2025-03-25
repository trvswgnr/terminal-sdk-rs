# Cart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | [**Vec<models::CartItem>**](CartItem.md) | An array of items in the current user's cart. | 
**subtotal** | **i32** | The subtotal of all items in the current user's cart, in cents (USD). | 
**address_id** | Option<**String**> | ID of the shipping address selected on the current user's cart. | [optional]
**card_id** | Option<**String**> | ID of the card selected on the current user's cart. | [optional]
**amount** | [**models::CartAmount**](Cart_amount.md) |  | 
**shipping** | Option<[**models::CartShipping**](Cart_shipping.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


