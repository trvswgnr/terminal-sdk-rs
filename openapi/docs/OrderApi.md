# \OrderApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_order**](OrderApi.md#get_order) | **GET** /order | List orders
[**get_order_by_id**](OrderApi.md#get_order_by_id) | **GET** /order/{id} | Get order
[**post_order**](OrderApi.md#post_order) | **POST** /order | Create order



## get_order

> models::GetOrder200Response get_order()
List orders

List the orders associated with the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetOrder200Response**](getOrder_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_by_id

> models::GetOrderById200Response get_order_by_id(id)
Get order

Get the order with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetOrderById200Response**](getOrderById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_order

> models::PostOrder200Response post_order(post_order_request)
Create order

Create an order without a cart. The order will be placed immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_order_request** | Option<[**PostOrderRequest**](PostOrderRequest.md)> |  |  |

### Return type

[**models::PostOrder200Response**](postOrder_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

