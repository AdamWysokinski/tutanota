import { OfflineMigration } from "../OfflineStorageMigrator.js"
import { OfflineStorage } from "../OfflineStorage.js"
import { SqlCipherFacade } from "../../../../native/common/generatedipc/SqlCipherFacade.js"

export const sys99: OfflineMigration = {
	app: "sys",
	version: 99,
	async migrate(storage: OfflineStorage, sqlCipherFacade: SqlCipherFacade) {
		//nothing to do because we only created a new type
	},
}
