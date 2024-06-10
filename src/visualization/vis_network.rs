use crate::model::state::network_state::UrbanNetworkState;
use crate::model::urban_network::edge::StreetEdgeLabel;
use crate::model::urban_network::node::StreetNode;
use krabmaga::bevy::prelude::*;
use krabmaga::engine::fields::network::{Edge, Network};
use krabmaga::engine::location::Real2D;
use krabmaga::visualization::fields::network::{EdgeRenderInfo, NetworkRender};

impl NetworkRender<StreetNode, StreetEdgeLabel, UrbanNetworkState> for UrbanNetworkState {
    fn get_network(state: &UrbanNetworkState) -> &Network<NetNode, String> {
        &state.network
    }

    fn get_edge_info(
        edge: &Edge<StreetEdgeLabel>,
        network: &Network<StreetNode, StreetEdgeLabel>,
    ) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            line_width: 1.,
            source_loc: network.get_object(edge.u).unwrap().loc,
            target_loc: network.get_object(edge.v).unwrap().loc,
            is_static: true,
        }
    }

    fn get_loc(network: &Network<StreetNode, StreetEdgeLabel>, node: u32) -> Real2D {
        network.get_object(node).unwrap().loc
    }
}
