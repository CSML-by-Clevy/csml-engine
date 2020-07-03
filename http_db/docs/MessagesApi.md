# \MessagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_message**](MessagesApi.md#add_message) | **post** /conversations/{conversationId}/messages | Add a message to an existing conversation
[**add_messages_bulk**](MessagesApi.md#add_messages_bulk) | **post** /conversations/{conversationId}/messages/bulk | Add an array of messages to a conversation



## add_message

> add_message(conversation_id, bot_id, user_id, channel_id, create_message_body)
Add a message to an existing conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_message_body** | [**CreateMessageBody**](CreateMessageBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_messages_bulk

> add_messages_bulk(conversation_id, bot_id, user_id, channel_id, create_message_body)
Add an array of messages to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_message_body** | [**Vec<crate::models::CreateMessageBody>**](CreateMessageBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

