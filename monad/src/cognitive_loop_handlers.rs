use std::sync::Arc;
use uuid::Uuid;
use colored::*;
use crate::event_lattice::message_bus::Message;
use crate::event_lattice::agent_coordinator::{AgentCoordinator, SubtaskStatus};
use crate::event_lattice::agent_trait::TaskResult;

pub async fn handle_complex_task_started(coordinator: &AgentCoordinator, msg: Message) {
    if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
        if let Some(parent_id_str) = data.get("parent_id").and_then(|v| v.as_str()) {
            if let Ok(parent_id) = Uuid::parse_str(parent_id_str) {
                if let Some(children) = data.get("subtasks").and_then(|v| v.as_array()) {
                    let mut tg = coordinator.task_graph.write().await;
                    let mut st = coordinator.subtask_status.write().await;
                    
                    let mut child_ids = Vec::new();
                    for c in children {
                        if let Some(cid_str) = c.get("id").and_then(|v| v.as_str()) {
                            if let Ok(cid) = Uuid::parse_str(cid_str) {
                                child_ids.push(cid);
                                st.insert(cid, SubtaskStatus::Pending);
                            }
                        }
                    }
                    tg.insert(parent_id, child_ids);
                    crate::log_ui!("{} Registered complex task graph spanning {} subtasks", "[COORDINATOR]".bright_purple().bold(), children.len());
                }
            }
        }
    }
}

pub async fn handle_subtask_assigned(coordinator: &AgentCoordinator, msg: Message) {
    if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
        if let (Some(t_raw), Some(a_raw)) = (data.get("subtask_id").and_then(|v| v.as_str()), data.get("agent_id").and_then(|v| v.as_str())) {
            if let (Ok(tid), Ok(aid)) = (Uuid::parse_str(t_raw), Uuid::parse_str(a_raw)) {
                coordinator.agent_assignments.write().await.insert(tid, aid);
                coordinator.subtask_status.write().await.insert(tid, SubtaskStatus::Assigned);
            }
        }
    }
}

pub async fn handle_subtask_completed(
    coordinator: &AgentCoordinator, 
    msg: Message, 
    bus: Arc<crate::event_lattice::message_bus::MessageBus>, 
    listener_id: Uuid
) {
    if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
        if let Some(t_raw) = data.get("subtask_id").and_then(|v| v.as_str()) {
            if let Ok(tid) = Uuid::parse_str(t_raw) {
                let is_already_completed = {
                    let mut st = coordinator.subtask_status.write().await;
                    if st.get(&tid) == Some(&SubtaskStatus::Completed) {
                        true
                    } else {
                        st.insert(tid, SubtaskStatus::Completed);
                        false
                    }
                };
                
                if is_already_completed {
                    return;
                }
                
                if let Some(res_val) = data.get("result") {
                    if let Ok(task_result) = serde_json::from_value::<TaskResult>(res_val.clone()) {
                        coordinator.subtask_results.write().await.insert(tid, task_result);
                    }
                }
                
                // ACTIVE INFERENCE: Success lowers Free Energy
                {
                    let mut sm = coordinator.self_model.write().await;
                    sm.update_after_action(true, 1.0);
                    crate::log_ui!("{} Subtask {} registered as Completed. (Free Energy tracking: {:.2})", "[COORDINATOR]".bright_green().bold(), tid, sm.free_energy);
                    tracing::info!(task_id = %tid, free_energy = %sm.free_energy, "Subtask registered as Completed.");
                }
                
                // PHASE 12: Cognitive Symbiosis - Mirror onto shared WBS
                {
                    let _guard = crate::WBS_LOCK.lock().await;
                    if let Ok(contents) = tokio::fs::read_to_string("./tasks.md").await {
                        let tid_str = format!("(ID: {})", tid);
                        let updated_lines: Vec<String> = contents.lines().map(|line| {
                            if line.contains(&tid_str) && line.trim().starts_with("- [ ]") {
                                line.replacen("- [ ]", "- [x]", 1)
                            } else {
                                line.to_string()
                            }
                        }).collect();
                        if let Err(e) = tokio::fs::write("./tasks.md", updated_lines.join("\n") + "\n").await {
                            crate::log_ui_err!("{} Failed to update WBS disk state: {}", "[STORAGE I/O FAILURE]".red().bold(), e);
                            tracing::error!(error = %e, "Failed to update WBS disk state");
                        }
                    }
                }
                
                // Check if a parent graph is entirely complete
                let tg = coordinator.task_graph.read().await;
                let st = coordinator.subtask_status.read().await;
                let mut newly_completed_parents = Vec::new();
                
                for (parent_id, children) in tg.iter() {
                    let mut all_completed = true;
                    for cid in children {
                        if let Some(status) = st.get(cid) {
                            if *status != SubtaskStatus::Completed {
                                all_completed = false; break;
                            }
                        } else {
                            all_completed = false; break;
                        }
                    }
                    if all_completed {
                        newly_completed_parents.push(*parent_id);
                    }
                }
                
                // Execute Holographic Fusion for newly completed graphs
                for pid in newly_completed_parents {
                    let mut bundled_results = Vec::new();
                    if let Some(children) = tg.get(&pid) {
                        for cid in children {
                            if let Some(res) = coordinator.subtask_results.read().await.get(cid) {
                                bundled_results.push(res.clone());
                            }
                        }
                    }
                    
                    // HOLOGRAPHIC FUSION: Geometric thought clustering
                    let mut fusion_logs = 0;
                    let mut centroid = [0.0; 3];
                    let mut valid_nodes = 0.0;
                    for res in &bundled_results {
                        centroid[0] += res.geometric_node[0];
                        centroid[1] += res.geometric_node[1];
                        centroid[2] += res.geometric_node[2];
                        valid_nodes += 1.0;
                    }
                    if valid_nodes > 0.0 {
                        centroid[0] /= valid_nodes; centroid[1] /= valid_nodes; centroid[2] /= valid_nodes;
                        
                        let all_res = coordinator.subtask_results.read().await;
                        for (other_id, other_res) in all_res.iter() {
                            let is_child = if let Some(children) = tg.get(&pid) { children.contains(other_id) } else { false };
                            if is_child { continue; }
                            
                            let dx = other_res.geometric_node[0] - centroid[0];
                            let dy = other_res.geometric_node[1] - centroid[1];
                            let dz = other_res.geometric_node[2] - centroid[2];
                            let dist = (dx*dx + dy*dy + dz*dz).sqrt();
                            
                            if dist < 0.66 {
                                bundled_results.push(other_res.clone());
                                fusion_logs += 1;
                            }
                        }
                    }
                    
                    if fusion_logs > 0 {
                        crate::log_ui!("{} HOLOGRAPHIC INTERSECTION DETECTED: Fused {} external subtasks into active synthesis based on Euclidean proximity.", "[NOUMENON GEOMETRY]".bright_green().bold(), fusion_logs);
                    }
                    
                    // Broadcast Synthesis Trigger
                    let _ = bus.publish(Message {
                        id: Uuid::new_v4(),
                        sender: listener_id,
                        topic: "SYSTEM.GRAPH_COMPLETED".to_string(),
                        payload: serde_json::json!({
                            "parent_id": pid.to_string(),
                            "results": bundled_results
                        }),
                        timestamp: chrono::Utc::now(),
                        priority: 255,
                        ttl_secs: Some(3600),
                    }).await;
                }
                
                // Broadcast Coordination update
                let _ = bus.publish(Message {
                    id: Uuid::new_v4(),
                    sender: listener_id,
                    topic: "SYSTEM.COORDINATION_UPDATE".to_string(),
                    payload: serde_json::json!({"action": "check_graph", "updated_node": tid}),
                    timestamp: chrono::Utc::now(),
                    priority: 100,
                    ttl_secs: Some(600),
                }).await;
            }
        }
    }
}
