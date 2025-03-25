# \AppOAuthApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_app_by_id**](AppOAuthApi.md#delete_app_by_id) | **DELETE** /app/{id} | Delete app
[**get_app**](AppOAuthApi.md#get_app) | **GET** /app | List apps
[**get_app_by_id**](AppOAuthApi.md#get_app_by_id) | **GET** /app/{id} | Get app
[**post_app**](AppOAuthApi.md#post_app) | **POST** /app | Create app



## delete_app_by_id

> models::DeleteAddressById200Response delete_app_by_id(id)
Delete app

Delete the app with the given ID.

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


## get_app

> models::GetApp200Response get_app()
List apps

List the current user's registered apps.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetApp200Response**](getApp_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_by_id

> models::GetAppById200Response get_app_by_id(id)
Get app

Get the app with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetAppById200Response**](getAppById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_app

> models::PostApp200Response post_app(post_app_request)
Create app

Create an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_app_request** | Option<[**PostAppRequest**](PostAppRequest.md)> |  |  |

### Return type

[**models::PostApp200Response**](postApp_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

