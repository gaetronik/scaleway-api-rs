# ScalewayK8sV1ClusterAutoscalerConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scale_down_disabled** | Option<**bool**> | Disable the cluster autoscaler | [optional]
**scale_down_delay_after_add** | Option<**String**> | How long after scale up that scale down evaluation resumes | [optional]
**estimator** | Option<**String**> | Type of resource estimator to be used in scale up | [optional][default to Estimator_UnknownEstimator]
**expander** | Option<**String**> | Type of node group expander to be used in scale up | [optional][default to Expander_UnknownExpander]
**ignore_daemonsets_utilization** | Option<**bool**> | Ignore DaemonSet pods when calculating resource utilization for scaling down | [optional]
**balance_similar_node_groups** | Option<**bool**> | Detect similar node groups and balance the number of nodes between them | [optional]
**expendable_pods_priority_cutoff** | Option<**f32**> | Pods with priority below cutoff will be expendable. They can be killed without any consideration during scale down and they don't cause scale up. Pods with null priority (PodPriority disabled) are non expendable. | [optional]
**scale_down_unneeded_time** | Option<**String**> | How long a node should be unneeded before it is eligible for scale down | [optional]
**scale_down_utilization_threshold** | Option<**f32**> | Node utilization level, defined as sum of requested resources divided by capacity, below which a node can be considered for scale down | [optional]
**max_graceful_termination_sec** | Option<**f32**> | Maximum number of seconds the cluster autoscaler waits for pod termination when trying to scale down a node | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


