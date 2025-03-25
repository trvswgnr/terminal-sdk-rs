# \SubscriptionApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subscription_by_id**](SubscriptionApi.md#delete_subscription_by_id) | **DELETE** /subscription/{id} | Cancel
[**get_subscription**](SubscriptionApi.md#get_subscription) | **GET** /subscription | List subscriptions
[**get_subscription_by_id**](SubscriptionApi.md#get_subscription_by_id) | **GET** /subscription/{id} | Get subscription
[**post_subscription**](SubscriptionApi.md#post_subscription) | **POST** /subscription | Subscribe



## delete_subscription_by_id

> models::DeleteAddressById200Response delete_subscription_by_id(id)
Cancel

Cancel a subscription for the current user.

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


## get_subscription

> models::GetSubscription200Response get_subscription()
List subscriptions

List the subscriptions associated with the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSubscription200Response**](getSubscription_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_by_id

> models::GetSubscriptionById200Response get_subscription_by_id(id)
Get subscription

Get the subscription with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetSubscriptionById200Response**](getSubscriptionById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_subscription

> models::DeleteAddressById200Response post_subscription(subscription)
Subscribe

Create a subscription for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription** | Option<[**Subscription**](Subscription.md)> |  |  |

### Return type

[**models::DeleteAddressById200Response**](deleteAddressById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

