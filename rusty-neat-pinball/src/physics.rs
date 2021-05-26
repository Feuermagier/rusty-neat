use rapier2d::{
    dynamics::{
        CCDSolver, IntegrationParameters, JointSet, RigidBody, RigidBodyHandle, RigidBodySet,
    },
    geometry::{BroadPhase, Collider, ColliderHandle, ColliderSet, NarrowPhase},
    na::Vector2,
    pipeline::{EventHandler, PhysicsHooks, PhysicsPipeline},
};

pub struct PhysicsEngine {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    physics_hooks: Box<dyn PhysicsHooks>,
    event_handler: Box<dyn EventHandler>,
}

impl PhysicsEngine {
    pub fn new(gravity: f32) -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0, gravity),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: Box::new(()),
            event_handler: Box::new(()),
        }
    }

    pub fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            self.physics_hooks.as_ref(),
            self.event_handler.as_ref(),
        )
    }

    pub(crate) fn get_body(&self, handle: RigidBodyHandle) -> &RigidBody {
        self.bodies.get(handle).unwrap()
    }

    pub(crate) fn get_body_mut(&mut self, handle: RigidBodyHandle) -> &mut RigidBody {
        self.bodies.get_mut(handle).unwrap()
    }

    pub(crate) fn get_collider(&self, handle: ColliderHandle) -> &Collider {
        self.colliders.get(handle).unwrap()
    }

    pub(crate) fn get_collider_mut(&mut self, handle: ColliderHandle) -> &mut Collider {
        self.colliders.get_mut(handle).unwrap()
    }

    pub(crate) fn add_body(&mut self, body: RigidBody) -> RigidBodyHandle {
        self.bodies.insert(body)
    }

    pub(crate) fn add_collider(
        &mut self,
        body_handle: RigidBodyHandle,
        collider: Collider,
    ) -> ColliderHandle {
        self.colliders
            .insert(collider, body_handle, &mut self.bodies)
    }
}
