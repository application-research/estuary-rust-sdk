# \CollectionsApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**collections_add_content_post**](CollectionsApi.md#collections_add_content_post) | **POST** /collections/add-content | Add contents to a collection
[**collections_content_coluuid_get**](CollectionsApi.md#collections_content_coluuid_get) | **GET** /collections/content/{coluuid} | Get contents in a collection
[**collections_create_post**](CollectionsApi.md#collections_create_post) | **POST** /collections/create | Create a new collection
[**collections_fs_add_post**](CollectionsApi.md#collections_fs_add_post) | **POST** /collections/fs/add | Add a file to a collection
[**collections_fs_list_get**](CollectionsApi.md#collections_fs_list_get) | **GET** /collections/fs/list | Create a new collection
[**collections_list_get**](CollectionsApi.md#collections_list_get) | **GET** /collections/list | List all collections



## collections_add_content_post

> ::std::collections::HashMap<String, String> collections_add_content_post(body)
Add contents to a collection

When a collection is created, users with valid API keys can add contents to the collection. This endpoint can be used to add contents to a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MainAddContentToCollectionParams**](MainAddContentToCollectionParams.md) | Contents to add to collection | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_content_coluuid_get

> String collections_content_coluuid_get(coluuid)
Get contents in a collection

This endpoint is used to get contents in a collection. When a collection is created, this collection object is retrievable along with its content via this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coluuid** | **String** | coluuid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_create_post

> crate::models::MainCollection collections_create_post(body)
Create a new collection

This endpoint is used to create a new collection. A collection is a representaion of a group of objects added on the estuary. This endpoint can be used to create a new collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MainCreateCollectionBody**](MainCreateCollectionBody.md) | Collection name and description | [required] |

### Return type

[**crate::models::MainCollection**](main.Collection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_fs_add_post

> String collections_fs_add_post(col, collection, content, path)
Add a file to a collection

This endpoint adds a file to a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**col** | **String** | Collection ID | [required] |
**collection** | **String** | Collection ID Long | [required] |
**content** | **String** | Content | [required] |
**path** | **String** | Path to file | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_fs_list_get

> String collections_fs_list_get(col, dir)
Create a new collection

This endpoint creates a new collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**col** | **String** | Collection | [required] |
**dir** | **String** | Directory | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## collections_list_get

> Vec<crate::models::MainCollection> collections_list_get()
List all collections

This endpoint is used to list all collections. Whenever a user logs on estuary, it will list all collections that the user has access to. This endpoint provides a way to list all collections to the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MainCollection>**](main.Collection.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

