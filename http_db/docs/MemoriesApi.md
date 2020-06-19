# \MemoriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_memories_bulk**](MemoriesApi.md#add_memories_bulk) | **post** /conversations/{conversationId}/memories/bulk | Add an array of memories to an existing conversation
[**add_memory**](MemoriesApi.md#add_memory) | **post** /conversations/{conversationId}/memories | Add a memory to an existing conversation
[**get_current_memories**](MemoriesApi.md#get_current_memories) | **get** /conversations/{conversationId}/memories/current | Get all the current memories
[**get_past_memories**](MemoriesApi.md#get_past_memories) | **get** /conversations/{conversationId}/memories/past | Get all the past memories (not in the current conversation)



## add_memories_bulk

> add_memories_bulk(conversation_id, bot_id, user_id, channel_id, create_memory_body)
Add an array of memories to an existing conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_memory_body** | [**Vec<crate::models::CreateMemoryBody>**](CreateMemoryBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_memory

> add_memory(conversation_id, bot_id, user_id, channel_id, create_memory_body)
Add a memory to an existing conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_memory_body** | [**CreateMemoryBody**](CreateMemoryBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_memories

> Vec<crate::models::MemoryModel> get_current_memories(conversation_id, bot_id, user_id, channel_id)
Get all the current memories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**Vec<crate::models::MemoryModel>**](MemoryModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_past_memories

> Vec<crate::models::MemoryModel> get_past_memories(conversation_id, bot_id, user_id, channel_id)
Get all the past memories (not in the current conversation)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**Vec<crate::models::MemoryModel>**](MemoryModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

