/* generated file, don't edit. */


import Foundation

/**
 * Operations for handling mobile payments.
 */
public protocol MobilePaymentsFacade {
	/**
	 * Display a pop-up for the user to start a subscription
	 */
	func requestSubscriptionToPlan(
		_ plan: String,
		_ interval: Int,
		_ customerIdBytes: DataWrapper
	) async throws -> MobilePaymentResult
	/**
	 * Returns a displayable price for a plan
	 */
	func getPlanPrice(
		_ plan: String,
		_ interval: Int
	) async throws -> String?
	/**
	 * Returns a displayable price for the current plan
	 */
	func getCurrentPlanPrice(
		_ customerIdBytes: DataWrapper
	) async throws -> String?
}
