# \MiscellaneousApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_view_init**](MiscellaneousApi.md#get_view_init) | **GET** /view/init | Get app data
[**post_email**](MiscellaneousApi.md#post_email) | **POST** /email | Subscribe email



## get_view_init

> models::GetViewInit200Response get_view_init()
Get app data

Get initial app data, including user, products, cart, addresses, cards, subscriptions, and orders.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetViewInit200Response**](getViewInit_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_email

> models::DeleteAddressById200Response post_email(post_email_request)
Subscribe email

Subscribe to email updates from Terminal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_email_request** | Option<[**PostEmailRequest**](PostEmailRequest.md)> |  |  |

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

