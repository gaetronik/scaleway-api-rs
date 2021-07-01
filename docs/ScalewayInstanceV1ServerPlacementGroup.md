# ScalewayInstanceV1ServerPlacementGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The placement group unique ID | [optional]
**name** | Option<**String**> | The placement group name | [optional]
**organization** | Option<**String**> | The placement group organization ID | [optional]
**project** | Option<**String**> | The placement group project ID | [optional]
**policy_mode** | Option<**String**> | Select the failling mode when the placement cannot be  respected, either optional or enforced | [optional][default to PolicyMode_Optional]
**policy_type** | Option<**String**> | Select the behavior of the placement group, either low_latency (group) or max_availability (spread) | [optional][default to PolicyType_MaxAvailability]
**policy_respected** | Option<**bool**> | Returns true if the policy is respected, false otherwise | [optional]
**zone** | Option<**String**> | The zone in which is the placement group | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

