# \PublicApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_by_cid_cid_get**](PublicApi.md#content_by_cid_cid_get) | **GET** /content/by-cid/{cid} | Get Content by Cid
[**public_by_cid_cid_get**](PublicApi.md#public_by_cid_cid_get) | **GET** /public/by-cid/{cid} | Get Content by Cid
[**public_info_get**](PublicApi.md#public_info_get) | **GET** /public/info | Get public node info
[**public_metrics_deals_on_chain_get**](PublicApi.md#public_metrics_deals_on_chain_get) | **GET** /public/metrics/deals-on-chain | Get deal metrics
[**public_miners_deals_miner_get**](PublicApi.md#public_miners_deals_miner_get) | **GET** /public/miners/deals/{miner} | Get all miners deals
[**public_miners_failures_miner_get**](PublicApi.md#public_miners_failures_miner_get) | **GET** /public/miners/failures/{miner} | Get all miners
[**public_miners_get**](PublicApi.md#public_miners_get) | **GET** /public/miners | Get all miners
[**public_miners_stats_miner_get**](PublicApi.md#public_miners_stats_miner_get) | **GET** /public/miners/stats/{miner} | Get miner stats
[**public_miners_storage_query_miner_get**](PublicApi.md#public_miners_storage_query_miner_get) | **GET** /public/miners/storage/query/{miner} | Get miner stats
[**public_net_addrs_get**](PublicApi.md#public_net_addrs_get) | **GET** /public/net/addrs | Net Addrs
[**public_net_peers_get**](PublicApi.md#public_net_peers_get) | **GET** /public/net/peers | Net Peers
[**public_stats_get**](PublicApi.md#public_stats_get) | **GET** /public/stats | Public stats



## content_by_cid_cid_get

> String content_by_cid_cid_get(cid)
Get Content by Cid

This endpoint returns the content associated with a CID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | Cid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_by_cid_cid_get

> String public_by_cid_cid_get(cid)
Get Content by Cid

This endpoint returns the content associated with a CID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** | Cid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_info_get

> String public_info_get()
Get public node info

This endpoint returns information about the node

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


## public_metrics_deals_on_chain_get

> String public_metrics_deals_on_chain_get()
Get deal metrics

This endpoint is used to get deal metrics

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


## public_miners_deals_miner_get

> String public_miners_deals_miner_get(miner)
Get all miners deals

This endpoint returns all miners deals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Filter by miner | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_miners_failures_miner_get

> String public_miners_failures_miner_get(miner)
Get all miners

This endpoint returns all miners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Filter by miner | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_miners_get

> String public_miners_get()
Get all miners

This endpoint returns all miners

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


## public_miners_stats_miner_get

> String public_miners_stats_miner_get(miner)
Get miner stats

This endpoint returns miner stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Filter by miner | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_miners_storage_query_miner_get

> String public_miners_storage_query_miner_get(miner)
Get miner stats

This endpoint returns miner storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Filter by miner | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_net_addrs_get

> Vec<String> public_net_addrs_get()
Net Addrs

This endpoint is used to get net addrs

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


## public_net_peers_get

> Vec<String> public_net_peers_get()
Net Peers

This endpoint is used to get net peers

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


## public_stats_get

> String public_stats_get()
Public stats

This endpoint is used to get public stats.

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

