// Copyright Istio Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::sync::Arc;

#[derive(Clone, Hash, Default, Debug, PartialEq, Eq, EncodeLabelSet)]
struct ProxyLabels {
    uid: String,
}

#[derive(Default)]
pub struct Metrics {
    pub(super) active_proxy_count: Family<(), Gauge>,
    pub(super) pending_proxy_count: Family<(), Gauge>,
    pub(super) proxies_started: Family<(), Counter>,
    pub(super) proxies_stopped: Family<(), Counter>,

    admin_handler: Arc<super::admin::WorkloadManagerAdminHandler>,
}

impl Metrics {
    pub fn new(registry: &mut Registry) -> Self {
        let m = Self::default();
        registry.register(
            "inpod_active_proxy_count",
            "The total number current workloads with active inpod proxies",
            m.active_proxy_count.clone(),
        );
        registry.register(
            "inpod_pending_proxy_count",
            "The total number current workloads with pending inpod proxies",
            m.pending_proxy_count.clone(),
        );
        registry.register(
            "inpod_proxies_started",
            "The total number of inpod proxies that were started",
            m.proxies_started.clone(),
        );
        registry.register(
            "inpod_proxies_stopped",
            "The total number of inpod proxies that were stopped",
            m.proxies_stopped.clone(),
        );
        m
    }
    pub fn admin_handler(&self) -> Arc<super::admin::WorkloadManagerAdminHandler> {
        self.admin_handler.clone()
    }
}