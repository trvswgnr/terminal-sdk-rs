# \CardApi

All URIs are relative to *https://api.dev.terminal.shop*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_card_by_id**](CardApi.md#delete_card_by_id) | **DELETE** /card/{id} | Delete card
[**get_card**](CardApi.md#get_card) | **GET** /card | List cards
[**get_card_by_id**](CardApi.md#get_card_by_id) | **GET** /card/{id} | Get card
[**post_card**](CardApi.md#post_card) | **POST** /card | Create card
[**post_card_collect**](CardApi.md#post_card_collect) | **POST** /card/collect | Collect card



## delete_card_by_id

> models::DeleteAddressById200Response delete_card_by_id(id)
Delete card

Delete a credit card associated with the current user.

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


## get_card

> models::GetCard200Response get_card()
List cards

List the credit cards associated with the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCard200Response**](getCard_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_by_id

> models::GetCardById200Response get_card_by_id(id)
Get card

Get a credit card by ID associated with the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetCardById200Response**](getCardById_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_card

> models::PostCard200Response post_card(post_card_request)
Create card

Attach a credit card (tokenized via Stripe) to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_card_request** | Option<[**PostCardRequest**](PostCardRequest.md)> |  |  |

### Return type

[**models::PostCard200Response**](postCard_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_card_collect

> models::PostCardCollect200Response post_card_collect()
Collect card

Create a temporary URL for collecting credit card information for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PostCardCollect200Response**](postCardCollect_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

