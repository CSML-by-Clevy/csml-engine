# \ConversationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_all_conversations**](ConversationsApi.md#close_all_conversations) | **post** /conversations/close | Close all conversations
[**close_conversation**](ConversationsApi.md#close_conversation) | **post** /conversations/{conversationId}/close | Close a specific conversations
[**create_conversation**](ConversationsApi.md#create_conversation) | **post** /conversations | Create a conversation
[**get_conversation**](ConversationsApi.md#get_conversation) | **get** /conversations/{conversationId} | Retrieve a conversation
[**get_latest_open**](ConversationsApi.md#get_latest_open) | **get** /conversations/latest | Get the latest open conversation
[**update_conversation**](ConversationsApi.md#update_conversation) | **put** /conversations/{conversationId} | Update a conversation (flow, step, last_interaction_at)



## close_all_conversations

> close_all_conversations(bot_id, user_id, channel_id, inline_object)
Close all conversations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**inline_object** | [**InlineObject**](InlineObject.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## close_conversation

> close_conversation(conversation_id, bot_id, user_id, channel_id, inline_object1)
Close a specific conversations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_conversation

> crate::models::ConversationModel create_conversation(bot_id, user_id, channel_id, create_conversation_body)
Create a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_conversation_body** | [**CreateConversationBody**](CreateConversationBody.md) |  | Required | 

### Return type

[**crate::models::ConversationModel**](ConversationModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation

> crate::models::ConversationModel get_conversation(conversation_id, bot_id, user_id, channel_id)
Retrieve a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::ConversationModel**](ConversationModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_open

> crate::models::InlineResponse200 get_latest_open(bot_id, user_id, channel_id)
Get the latest open conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_conversation

> update_conversation(conversation_id, bot_id, user_id, channel_id, update_conversation_body)
Update a conversation (flow, step, last_interaction_at)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**update_conversation_body** | [**UpdateConversationBody**](UpdateConversationBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

