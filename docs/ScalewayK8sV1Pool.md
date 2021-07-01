# ScalewayK8sV1Pool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the pool | [optional]
**cluster_id** | Option<**String**> | The cluster ID of the pool | [optional]
**created_at** | Option<**String**> | The date at which the pool was created | [optional]
**updated_at** | Option<**String**> | The date at which the pool was last updated | [optional]
**name** | Option<**String**> | The name of the pool | [optional]
**status** | Option<**String**> | The status of the pool | [optional][default to Status_Unknown]
**version** | Option<**String**> | The version of the pool | [optional]
**node_type** | **String** | The node type is the type of Scaleway Instance wanted for the pool | 
**autoscaling** | Option<**bool**> | The enablement of the autoscaling feature for the pool | [optional]
**size** | **f32** | The size (number of nodes) of the pool | 
**min_size** | Option<**f32**> | The minimun size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**max_size** | Option<**f32**> | The maximum size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**container_runtime** | Option<**String**> | The customization of the container runtime is available for each pool. Note that `docker` is the only supporter runtime at the moment. Others are to be considered experimental.  | [optional][default to ContainerRuntime_UnknownRuntime]
**autohealing** | Option<**bool**> | The enablement of the autohealing feature for the pool | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the pool | [optional]
**placement_group_id** | Option<**String**> | The placement group ID in which all the nodes of the pool will be created | [optional]
**kubelet_args** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The Kubelet arguments to be used by this pool. Note that this feature is to be considered as experimental | [optional]
**upgrade_policy** | Option<[**crate::models::ScalewayK8sV1PoolUpgradePolicy**](scaleway_k8s_v1_Pool_upgrade_policy.md)> |  | [optional]
**zone** | Option<**String**> | The Zone in which the Pool's node will be spawn in | [optional]
**region** | Option<**String**> | The cluster region of the pool | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

