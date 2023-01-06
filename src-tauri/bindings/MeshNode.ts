// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { MeshNodeDeviceMetrics } from "./MeshNodeDeviceMetrics";
import type { MeshNodeEnvironmentMetrics } from "./MeshNodeEnvironmentMetrics";
import type { NodeInfo } from "./protobufs/NodeInfo";

export interface MeshNode { deviceMetrics: Array<MeshNodeDeviceMetrics>, environmentMetrics: Array<MeshNodeEnvironmentMetrics>, data: NodeInfo, }