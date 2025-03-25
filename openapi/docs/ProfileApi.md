# \ProfileApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile**](ProfileApi.md#get_profile) | **GET** /profile | Get profile
[**put_profile**](ProfileApi.md#put_profile) | **PUT** /profile | Update profile



## get_profile

> models::GetProfile200Response get_profile()
Get profile

Get the current user's profile.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetProfile200Response**](getProfile_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_profile

> models::PutProfile200Response put_profile(put_profile_request)
Update profile

Update the current user's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_profile_request** | Option<[**PutProfileRequest**](PutProfileRequest.md)> |  |  |

### Return type

[**models::PutProfile200Response**](putProfile_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

