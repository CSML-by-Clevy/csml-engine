# \StateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_full_state**](StateApi.md#clear_full_state) | **delete** /state | Clear a client's full memory state
[**delete_item_state**](StateApi.md#delete_item_state) | **delete** /state/{compositeKey} | Delete a state item
[**get_full_state**](StateApi.md#get_full_state) | **get** /state | Get the full state for a client
[**get_item_state**](StateApi.md#get_item_state) | **get** /state/{compositeKey} | Get the state of the requested memory item. If it does not exist yet, a success code of 204 is returned
[**set_item_state**](StateApi.md#set_item_state) | **put** /state/{compositeKey} | Set a memory item to a given state



## clear_full_state

> clear_full_state(bot_id, user_id, channel_id)
Clear a client's full memory state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_item_state

> delete_item_state(composite_key, bot_id, user_id, channel_id)
Delete a state item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**composite_key** | **String** | compositeKey (id) of memory to retrieve the state from | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_full_state

> Vec<crate::models::StateModel> get_full_state(bot_id, user_id, channel_id)
Get the full state for a client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**Vec<crate::models::StateModel>**](StateModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_state

> crate::models::StateModel get_item_state(composite_key, bot_id, user_id, channel_id)
Get the state of the requested memory item. If it does not exist yet, a success code of 204 is returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**composite_key** | **String** | compositeKey (id) of memory to retrieve the state from | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::StateModel**](StateModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_item_state

> crate::models::StateModel set_item_state(composite_key, bot_id, user_id, channel_id, set_state_body)
Set a memory item to a given state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**composite_key** | **String** | compositeKey (id) of memory to retrieve the state from | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**set_state_body** | [**SetStateBody**](SetStateBody.md) |  | Required | 

### Return type

[**crate::models::StateModel**](StateModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

