# \NodesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_node**](NodesApi.md#create_node) | **post** /conversations/{conversationId}/nodes | Add a new node to a conversation
[**get_conversation_nodes**](NodesApi.md#get_conversation_nodes) | **get** /conversations/{conversationId}/nodes | Retrieve a conversation's nodes



## create_node

> create_node(conversation_id, bot_id, user_id, channel_id, create_node_body)
Add a new node to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_node_body** | [**CreateNodeBody**](CreateNodeBody.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_conversation_nodes

> Vec<crate::models::NodeModel> get_conversation_nodes(conversation_id, bot_id, user_id, channel_id)
Retrieve a conversation's nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | ID of conversation | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**Vec<crate::models::NodeModel>**](NodeModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

