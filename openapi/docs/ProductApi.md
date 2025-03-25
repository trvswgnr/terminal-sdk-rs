# \ProductApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_product**](ProductApi.md#get_product) | **GET** /product | List products
[**get_product_by_id**](ProductApi.md#get_product_by_id) | **GET** /product/{id} | Get product



## get_product

> models::GetProduct200Response get_product()
List products

List all products for sale in the Terminal shop.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetProduct200Response**](getProduct_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_by_id

> models::GetProductById200Response get_product_by_id(id)
Get product

Get a product by ID from the Terminal shop.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetProductById200Response**](getProductById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

