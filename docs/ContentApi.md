# \ContentApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_add_car_post**](ContentApi.md#content_add_car_post) | **POST** /content/add-car | Add Car object
[**content_add_ipfs_post**](ContentApi.md#content_add_ipfs_post) | **POST** /content/add-ipfs | Add IPFS object
[**content_add_post**](ContentApi.md#content_add_post) | **POST** /content/add | Add new content
[**content_aggregated_content_get**](ContentApi.md#content_aggregated_content_get) | **GET** /content/aggregated/{content} | Get aggregated content stats
[**content_all_deals_get**](ContentApi.md#content_all_deals_get) | **GET** /content/all-deals | Get all deals for a user
[**content_bw_usage_content_get**](ContentApi.md#content_bw_usage_content_get) | **GET** /content/bw-usage/{content} | Get content bandwidth
[**content_create_post**](ContentApi.md#content_create_post) | **POST** /content/create | Add a new content
[**content_deals_get**](ContentApi.md#content_deals_get) | **GET** /content/deals | Content with deals
[**content_ensure_replication_datacid_get**](ContentApi.md#content_ensure_replication_datacid_get) | **GET** /content/ensure-replication/{datacid} | Ensure Replication
[**content_failures_content_get**](ContentApi.md#content_failures_content_get) | **GET** /content/failures/{content} | List all failures for a content
[**content_importdeal_post**](ContentApi.md#content_importdeal_post) | **POST** /content/importdeal | Import a deal
[**content_list_get**](ContentApi.md#content_list_get) | **GET** /content/list | List all pinned content
[**content_read_cont_get**](ContentApi.md#content_read_cont_get) | **GET** /content/read/{cont} | Read content
[**content_staging_zones_get**](ContentApi.md#content_staging_zones_get) | **GET** /content/staging-zones | Get staging zone for user
[**content_stats_get**](ContentApi.md#content_stats_get) | **GET** /content/stats | Get content statistics
[**content_status_id_get**](ContentApi.md#content_status_id_get) | **GET** /content/status/{id} | Content Status



## content_add_car_post

> String content_add_car_post(body, filename, commp, size)
Add Car object

This endpoint is used to add a car object to the network. The object can be a file or a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Car | [required] |
**filename** | Option<**String**> | Filename |  |
**commp** | Option<**String**> | Commp |  |
**size** | Option<**String**> | Size |  |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_add_ipfs_post

> String content_add_ipfs_post(body)
Add IPFS object

This endpoint is used to add an IPFS object to the network. The object can be a file or a directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UtilContentAddIpfsBody**](UtilContentAddIpfsBody.md) | IPFS Body | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_add_post

> String content_add_post(file, collection, collection_path)
Add new content

This endpoint is used to upload new content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | File to upload | [required] |
**collection** | Option<**String**> | Collection |  |
**collection_path** | Option<**String**> | Collection Path |  |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_aggregated_content_get

> String content_aggregated_content_get(content)
Get aggregated content stats

This endpoint returns aggregated content stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content** | **String** | Content ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_all_deals_get

> String content_all_deals_get(begin, duration, all)
Get all deals for a user

This endpoint is used to get all deals for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**begin** | **String** | Begin | [required] |
**duration** | **String** | Duration | [required] |
**all** | **String** | All | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_bw_usage_content_get

> String content_bw_usage_content_get(content)
Get content bandwidth

This endpoint returns content bandwidth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content** | **String** | Content ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_create_post

> String content_create_post(body)
Add a new content

This endpoint adds a new content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | Content | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_deals_get

> String content_deals_get(limit, offset)
Content with deals

This endpoint lists all content with deals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Limit |  |
**offset** | Option<**i32**> | Offset |  |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_ensure_replication_datacid_get

> String content_ensure_replication_datacid_get(datacid)
Ensure Replication

This endpoint ensures that the content is replicated to the specified number of providers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datacid** | **String** | Data CID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_failures_content_get

> String content_failures_content_get(content)
List all failures for a content

This endpoint returns all failures for a content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content** | **String** | Content ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_importdeal_post

> String content_importdeal_post(body)
Import a deal

This endpoint imports a deal into the shuttle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MainImportDealBody**](MainImportDealBody.md) | Import a deal | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_list_get

> Vec<String> content_list_get()
List all pinned content

This endpoint lists all content

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_read_cont_get

> String content_read_cont_get(cont)
Read content

This endpoint reads content from the blockstore

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cont** | **String** | CID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_staging_zones_get

> String content_staging_zones_get()
Get staging zone for user

This endpoint is used to get staging zone for user.

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


## content_stats_get

> String content_stats_get(limit)
Get content statistics

This endpoint is used to get content statistics. Every content stored in the network (estuary) is tracked by a unique ID which can be used to get information about the content. This endpoint will allow the consumer to get the collected stats of a conten

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **String** | limit | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_status_id_get

> String content_status_id_get(id)
Content Status

This endpoint returns the status of a content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Content ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

