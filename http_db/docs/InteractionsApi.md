# \InteractionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_interaction**](InteractionsApi.md#get_interaction) | **get** /interactions/{interactionId} | Get an interaction
[**get_interaction_status**](InteractionsApi.md#get_interaction_status) | **get** /interactions/{interactionId}/status | Find if a given interaction was successfully processed
[**get_lock_status**](InteractionsApi.md#get_lock_status) | **get** /interactions/lock | Find if the current user is currently processing an interaction
[**init_interaction**](InteractionsApi.md#init_interaction) | **post** /interactions | Initialize a new interaction
[**update_interaction**](InteractionsApi.md#update_interaction) | **put** /interactions/{interactionId} | Update an interaction's status



## get_interaction

> crate::models::InteractionModel get_interaction(interaction_id, bot_id, user_id, channel_id)
Get an interaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interaction_id** | **String** | ID of interaction | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::InteractionModel**](InteractionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_interaction_status

> crate::models::InlineResponse2001 get_interaction_status(interaction_id, bot_id, user_id, channel_id)
Find if a given interaction was successfully processed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interaction_id** | **String** | ID of interaction | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lock_status

> crate::models::InlineResponse2002 get_lock_status(bot_id, user_id, channel_id)
Find if the current user is currently processing an interaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## init_interaction

> crate::models::InteractionModel init_interaction(bot_id, user_id, channel_id, create_interaction_body)
Initialize a new interaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**create_interaction_body** | [**CreateInteractionBody**](CreateInteractionBody.md) |  | Required | 

### Return type

[**crate::models::InteractionModel**](InteractionModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_interaction

> update_interaction(interaction_id, bot_id, user_id, channel_id, inline_object2)
Update an interaction's status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interaction_id** | **String** | ID of interaction | Required | 
**bot_id** | **String** | ID of bot | Required | 
**user_id** | **String** | ID of user | Required | 
**channel_id** | **String** | ID of channel | Required | 
**inline_object2** | [**InlineObject2**](InlineObject2.md) |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

