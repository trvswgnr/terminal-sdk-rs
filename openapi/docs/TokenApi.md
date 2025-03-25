# \TokenApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_token_by_id**](TokenApi.md#delete_token_by_id) | **DELETE** /token/{id} | Delete token
[**get_token**](TokenApi.md#get_token) | **GET** /token | List tokens
[**get_token_by_id**](TokenApi.md#get_token_by_id) | **GET** /token/{id} | Get token
[**post_token**](TokenApi.md#post_token) | **POST** /token | Create token



## delete_token_by_id

> models::DeleteAddressById200Response delete_token_by_id(id)
Delete token

Delete the personal access token with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token

> models::GetToken200Response get_token()
List tokens

List the current user's personal access tokens.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetToken200Response**](getToken_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_by_id

> models::GetTokenById200Response get_token_by_id(id)
Get token

Get the personal access token with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetTokenById200Response**](getTokenById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_token

> models::PostToken200Response post_token()
Create token

Create a personal access token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PostToken200Response**](postToken_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

