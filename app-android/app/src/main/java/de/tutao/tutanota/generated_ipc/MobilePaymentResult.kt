/* generated file, don't edit. */


package de.tutao.tutanota.ipc

import kotlinx.serialization.*
import kotlinx.serialization.json.*


@Serializable
data class MobilePaymentResult(
	val success: Boolean,
	val transactionID: String?,
	val transactionHash: String?,
)
