# \AddressApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_address_by_id**](AddressApi.md#delete_address_by_id) | **DELETE** /address/{id} | Delete address
[**get_address**](AddressApi.md#get_address) | **GET** /address | Get addresses
[**get_address_by_id**](AddressApi.md#get_address_by_id) | **GET** /address/{id} | Get address
[**post_address**](AddressApi.md#post_address) | **POST** /address | Create address



## delete_address_by_id

> models::DeleteAddressById200Response delete_address_by_id(id)
Delete address

Delete a shipping address from the current user.

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


## get_address

> models::GetAddress200Response get_address()
Get addresses

Get the shipping addresses associated with the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAddress200Response**](getAddress_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_address_by_id

> models::GetAddressById200Response get_address_by_id(id)
Get address

Get the shipping address with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetAddressById200Response**](getAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_address

> models::PostAddress200Response post_address(post_address_request)
Create address

Create and add a shipping address to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_address_request** | Option<[**PostAddressRequest**](PostAddressRequest.md)> |  |  |

### Return type

[**models::PostAddress200Response**](postAddress_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

