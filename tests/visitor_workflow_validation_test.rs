use design_patterns_rust::patterns::gof::behavioral::visitor::workflow_validation::{
    Workflow, WorkflowValidationVisitor,
};

#[test]
fn visitor_validates_complete_workflow() {
    let workflow = Workflow::new("purchase")
        .with_start("start")
        .with_task("collect_request", "buyer")
        .with_approval("manager_approval", "manager")
        .with_end("done");
    let mut visitor = WorkflowValidationVisitor::default();

    workflow.accept(&mut visitor);

    assert!(visitor.is_valid());
    assert_eq!(
        visitor.visited_nodes(),
        [
            "workflow:purchase",
            "start:start",
            "task:collect_request",
            "approval:manager_approval",
            "end:done"
        ]
    );
}

#[test]
fn visitor_collects_workflow_validation_errors() {
    let workflow = Workflow::new("broken")
        .with_task("collect_request", "")
        .with_approval("manager_approval", "");
    let mut visitor = WorkflowValidationVisitor::default();

    workflow.accept(&mut visitor);

    assert!(!visitor.is_valid());
    assert_eq!(
        visitor.errors(),
        [
            "workflow missing start node",
            "workflow missing end node",
            "task collect_request missing owner",
            "approval manager_approval missing approver"
        ]
    );
}

#[test]
fn visitor_detects_duplicate_workflow_node_ids() {
    let workflow = Workflow::new("duplicates")
        .with_start("start")
        .with_task("review", "ops")
        .with_approval("review", "lead")
        .with_end("done");
    let mut visitor = WorkflowValidationVisitor::default();

    workflow.accept(&mut visitor);

    assert_eq!(visitor.errors(), ["duplicate node id review"]);
}
