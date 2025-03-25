# \CartApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_cart**](CartApi.md#delete_cart) | **DELETE** /cart | Clear cart
[**get_cart**](CartApi.md#get_cart) | **GET** /cart | Get cart
[**post_cart_convert**](CartApi.md#post_cart_convert) | **POST** /cart/convert | Convert to order
[**put_cart_address**](CartApi.md#put_cart_address) | **PUT** /cart/address | Set address
[**put_cart_card**](CartApi.md#put_cart_card) | **PUT** /cart/card | Set card
[**put_cart_item**](CartApi.md#put_cart_item) | **PUT** /cart/item | Add item



## delete_cart

> models::DeleteAddressById200Response delete_cart()
Clear cart

Clear the current user's cart.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cart

> models::GetCart200Response get_cart()
Get cart

Get the current user's cart.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCart200Response**](getCart_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cart_convert

> models::PostCartConvert200Response post_cart_convert()
Convert to order

Convert the current user's cart to an order.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PostCartConvert200Response**](postCartConvert_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_cart_address

> models::DeleteAddressById200Response put_cart_address(put_cart_address_request)
Set address

Set the shipping address for the current user's cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_cart_address_request** | Option<[**PutCartAddressRequest**](PutCartAddressRequest.md)> |  |  |

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_cart_card

> models::DeleteAddressById200Response put_cart_card(put_cart_card_request)
Set card

Set the credit card for the current user's cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_cart_card_request** | Option<[**PutCartCardRequest**](PutCartCardRequest.md)> |  |  |

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_cart_item

> models::PutCartItem200Response put_cart_item(put_cart_item_request)
Add item

Add an item to the current user's cart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_cart_item_request** | Option<[**PutCartItemRequest**](PutCartItemRequest.md)> |  |  |

### Return type

[**models::PutCartItem200Response**](putCartItem_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

