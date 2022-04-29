# \UserApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_api_keys_get**](UserApi.md#user_api_keys_get) | **GET** /user/api-keys | Get API keys for a user
[**user_api_keys_key_delete**](UserApi.md#user_api_keys_key_delete) | **DELETE** /user/api-keys/{key} | Revoke a User API Key.
[**user_api_keys_post**](UserApi.md#user_api_keys_post) | **POST** /user/api-keys | Create API keys for a user
[**user_export_get**](UserApi.md#user_export_get) | **GET** /user/export | Export user data
[**user_stats_get**](UserApi.md#user_stats_get) | **GET** /user/stats | Create API keys for a user



## user_api_keys_get

> Vec<crate::models::MainGetApiKeysResp> user_api_keys_get()
Get API keys for a user

This endpoint is used to get API keys for a user. In estuary, each user can be given multiple API keys (tokens). This endpoint can be used to retrieve all available API keys for a given user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MainGetApiKeysResp>**](main.getApiKeysResp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_api_keys_key_delete

> String user_api_keys_key_delete(key)
Revoke a User API Key.

This endpoint is used to revoke a user API key. In estuary, every user is assigned with an API key, this API key is generated and issued for each user and is primarily use to access all estuary features. This endpoint can be used to revoke the API key thats assigned to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Key | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_api_keys_post

> crate::models::MainGetApiKeysResp user_api_keys_post(perm, expiry)
Create API keys for a user

This endpoint is used to create API keys for a user. In estuary, each user is given an API key to access all features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**perm** | Option<**String**> | perm |  |
**expiry** | Option<**String**> | expiry |  |

### Return type

[**crate::models::MainGetApiKeysResp**](main.getApiKeysResp.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_export_get

> String user_export_get()
Export user data

This endpoint is used to get API keys for a user.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_stats_get

> crate::models::MainUserStatsResponse user_stats_get()
Create API keys for a user

This endpoint is used to create API keys for a user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MainUserStatsResponse**](main.userStatsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

