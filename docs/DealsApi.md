# \DealsApi

All URIs are relative to *http://api.estuary.tech*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deals_estimate_post**](DealsApi.md#deals_estimate_post) | **POST** /deals/estimate | Estimate the cost of a deal
[**deals_failures_get**](DealsApi.md#deals_failures_get) | **GET** /deals/failures | Get storage failures
[**deals_info_dealid_get**](DealsApi.md#deals_info_dealid_get) | **GET** /deals/info/{dealid} | Get Deal Info
[**deals_make_miner_post**](DealsApi.md#deals_make_miner_post) | **POST** /deals/make/{miner} | Make Deal
[**deals_proposal_propcid_get**](DealsApi.md#deals_proposal_propcid_get) | **GET** /deals/proposal/{propcid} | Get Proposal
[**deals_query_miner_get**](DealsApi.md#deals_query_miner_get) | **GET** /deals/query/{miner} | Query Ask
[**deals_status_by_proposal_propcid_get**](DealsApi.md#deals_status_by_proposal_propcid_get) | **GET** /deals/status-by-proposal/{propcid} | Get Deal Status by PropCid
[**deals_status_deal_get**](DealsApi.md#deals_status_deal_get) | **GET** /deals/status/{deal} | Get Deal Status
[**deals_status_miner_propcid_get**](DealsApi.md#deals_status_miner_propcid_get) | **GET** /deals/status/{miner}/{propcid} | Deal Status
[**deals_transfer_in_progress_get**](DealsApi.md#deals_transfer_in_progress_get) | **GET** /deals/transfer/in-progress | Transfer In Progress
[**deals_transfer_status_post**](DealsApi.md#deals_transfer_status_post) | **POST** /deals/transfer/status | Transfer Status



## deals_estimate_post

> String deals_estimate_post(body)
Estimate the cost of a deal

This endpoint estimates the cost of a deal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MainEstimateDealBody**](MainEstimateDealBody.md) | The size of the deal in bytes, the replication factor, and the duration of the deal in blocks | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_failures_get

> String deals_failures_get()
Get storage failures

This endpoint returns a list of storage failures

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


## deals_info_dealid_get

> String deals_info_dealid_get(dealid)
Get Deal Info

This endpoint returns the deal info for a deal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dealid** | **i32** | Deal ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_make_miner_post

> String deals_make_miner_post(miner, deal_request)
Make Deal

This endpoint makes a deal for a given content and miner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Miner | [required] |
**deal_request** | **String** | Deal Request | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_proposal_propcid_get

> String deals_proposal_propcid_get(propcid)
Get Proposal

This endpoint returns the proposal for a deal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**propcid** | **String** | Proposal CID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_query_miner_get

> String deals_query_miner_get(miner)
Query Ask

This endpoint returns the ask for a given CID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | CID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_status_by_proposal_propcid_get

> String deals_status_by_proposal_propcid_get(propcid)
Get Deal Status by PropCid

Get Deal Status by PropCid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**propcid** | **String** | PropCid | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_status_deal_get

> String deals_status_deal_get(deal)
Get Deal Status

This endpoint returns the status of a deal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal** | **i32** | Deal ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_status_miner_propcid_get

> String deals_status_miner_propcid_get(miner, propcid)
Deal Status

This endpoint returns the status of a deal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner** | **String** | Miner | [required] |
**propcid** | **String** | Proposal CID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deals_transfer_in_progress_get

> String deals_transfer_in_progress_get()
Transfer In Progress

This endpoint returns the in-progress transfers

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


## deals_transfer_status_post

> String deals_transfer_status_post()
Transfer Status

This endpoint returns the status of a transfer

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

