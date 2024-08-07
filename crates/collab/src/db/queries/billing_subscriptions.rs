use crate::db::billing_subscription::StripeSubscriptionStatus;

use super::*;

#[derive(Debug)]
pub struct CreateBillingSubscriptionParams {
    pub user_id: UserId,
    pub stripe_customer_id: String,
    pub stripe_subscription_id: String,
    pub stripe_subscription_status: StripeSubscriptionStatus,
}

impl Database {
    /// Creates a new billing subscription.
    pub async fn create_billing_subscription(
        &self,
        params: &CreateBillingSubscriptionParams,
    ) -> Result<()> {
        self.transaction(|tx| async move {
            billing_subscription::Entity::insert(billing_subscription::ActiveModel {
                user_id: ActiveValue::set(params.user_id),
                stripe_customer_id: ActiveValue::set(params.stripe_customer_id.clone()),
                stripe_subscription_id: ActiveValue::set(params.stripe_subscription_id.clone()),
                stripe_subscription_status: ActiveValue::set(params.stripe_subscription_status),
                ..Default::default()
            })
            .exec_without_returning(&*tx)
            .await?;

            Ok(())
        })
        .await
    }

    /// Returns the billing subscription with the specified ID.
    pub async fn get_billing_subscription_by_id(
        &self,
        id: BillingSubscriptionId,
    ) -> Result<Option<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            Ok(billing_subscription::Entity::find_by_id(id)
                .one(&*tx)
                .await?)
        })
        .await
    }

    /// Returns all of the billing subscriptions for the user with the specified ID.
    ///
    /// Note that this returns the subscriptions regardless of their status.
    /// If you're wanting to check if a use has an active billing subscription,
    /// use `get_active_billing_subscriptions` instead.
    pub async fn get_billing_subscriptions(
        &self,
        user_id: UserId,
    ) -> Result<Vec<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            let subscriptions = billing_subscription::Entity::find()
                .filter(billing_subscription::Column::UserId.eq(user_id))
                .order_by_asc(billing_subscription::Column::Id)
                .all(&*tx)
                .await?;

            Ok(subscriptions)
        })
        .await
    }

    /// Returns all of the active billing subscriptions for the user with the specified ID.
    pub async fn get_active_billing_subscriptions(
        &self,
        user_id: UserId,
    ) -> Result<Vec<billing_subscription::Model>> {
        self.transaction(|tx| async move {
            let subscriptions = billing_subscription::Entity::find()
                .filter(
                    billing_subscription::Column::UserId.eq(user_id).and(
                        billing_subscription::Column::StripeSubscriptionStatus
                            .eq(StripeSubscriptionStatus::Active),
                    ),
                )
                .order_by_asc(billing_subscription::Column::Id)
                .all(&*tx)
                .await?;

            Ok(subscriptions)
        })
        .await
    }
}
