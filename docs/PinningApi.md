# \PinningApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pinning_pins_get**](PinningApi.md#pinning_pins_get) | **GET** /pinning/pins | List all pinned objects
[**pinning_pins_post**](PinningApi.md#pinning_pins_post) | **POST** /pinning/pins | Add and pin object
[**pinning_pins_requestid_delete**](PinningApi.md#pinning_pins_requestid_delete) | **DELETE** /pinning/pins/{requestid} | Delete a pinned object
[**pinning_pins_requestid_get**](PinningApi.md#pinning_pins_requestid_get) | **GET** /pinning/pins/{requestid} | Get a pinned objects
[**pinning_pins_requestid_post**](PinningApi.md#pinning_pins_requestid_post) | **POST** /pinning/pins/{requestid} | Replace a pinned object



## pinning_pins_get

> pinning_pins_get()
List all pinned objects

This endpoint lists all pinned objects

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinning_pins_post

> String pinning_pins_post(cid, name)
Add and pin object

This endpoint adds a pin to the IPFS daemon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | cid | [required] |
**name** | **String** | name | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinning_pins_requestid_delete

> String pinning_pins_requestid_delete(requestid)
Delete a pinned object

This endpoint deletes a pinned object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requestid** | **String** | requestid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinning_pins_requestid_get

> String pinning_pins_requestid_get(requestid)
Get a pinned objects

This endpoint returns a pinned object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requestid** | **String** | cid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinning_pins_requestid_post

> String pinning_pins_requestid_post(requestid)
Replace a pinned object

This endpoint replaces a pinned object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**requestid** | **String** | id | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

