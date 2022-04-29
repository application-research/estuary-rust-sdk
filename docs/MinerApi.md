# \MinerApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**public_miners_deals_miner_get**](MinerApi.md#public_miners_deals_miner_get) | **GET** /public/miners/deals/{miner} | Get all miners deals
[**public_miners_stats_miner_get**](MinerApi.md#public_miners_stats_miner_get) | **GET** /public/miners/stats/{miner} | Get miner stats



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

