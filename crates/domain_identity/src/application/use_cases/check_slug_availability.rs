use std::sync::Arc;

use crate::domain::entities::tenant::validate_slug;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct CheckSlugAvailabilityCommand {
    pub slug: String,
}

#[derive(Debug)]
pub struct SlugAvailability {
    pub slug: String,
    pub available: bool,
    /// Set when `available` is false because the slug is malformed (not just taken).
    pub reason: Option<String>,
}

pub struct CheckSlugAvailabilityUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl CheckSlugAvailabilityUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    /// Returns whether the given slug is currently available. A slug is
    /// available when:
    ///   1. It passes domain validation (format + not reserved)
    ///   2. No existing tenant owns it
    ///
    /// Format errors are returned as `available=false` with a `reason`, NOT
    /// as a domain error — this endpoint is meant to power a real-time
    /// availability check while the user types in the registration form.
    #[tracing::instrument(name = "check_slug_availability", skip(self, command), fields(slug = %command.slug))]
    pub async fn execute(
        &self,
        command: CheckSlugAvailabilityCommand,
    ) -> Result<SlugAvailability, DomainError> {
        if let Err(DomainError::InvalidSlug(reason)) = validate_slug(&command.slug) {
            return Ok(SlugAvailability {
                slug: command.slug,
                available: false,
                reason: Some(reason),
            });
        }

        let mut uow = self.uow_manager.begin().await?;
        let taken = uow.tenants().find_by_slug(&command.slug).await?.is_some();
        uow.commit().await?;

        Ok(SlugAvailability {
            slug: command.slug,
            available: !taken,
            reason: if taken {
                Some("slug já em uso por outra empresa.".to_string())
            } else {
                None
            },
        })
    }
}
