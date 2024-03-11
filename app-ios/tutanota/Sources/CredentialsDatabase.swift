class CredentialsDatabase {
	private let db: SqliteDb

	init(db: SqliteDb) throws {
		self.db = db
		let dbPath = makeDbPath(fileName: "credentials.sqlite")
		try db.open(dbPath: dbPath.absoluteString)
		try self.createCredentialTable()
	}

	func createCredentialTable() throws {
		try db.prepare(
			query: """
				CREATE TABLE IF NOT EXISTS credentials
				(login TEXT NOT NULL,
				userId TEXT NOT NULL,
				type TEXT NOT NULL,
				accessToken TEXT NOT NULL,
				databaseKey TEXT,
				encryptedPassword TEXT NOT NULL,
				PRIMARY KEY (userId),
				UNIQUE(login))
				"""
		)
		.run()
	}

	func getAll() throws -> [PersistedCredentials] {
		try db.prepare(
			query: """
				SELECT * FROM credentials
				"""
		)
		.all()
		.map { sqlRow in
			let credentialsInfo = CredentialsInfo(
				login: try sqlRow["login"]!.asString(),
				userId: try sqlRow["userId"]!.asString(),
				type: CredentialType(rawValue: try sqlRow["type"]!.asString())!
			)

			let databaseKey: String? = if case let .string(value) = sqlRow["databaseKey"] { value } else { nil }
			return PersistedCredentials(
				credentialInfo: credentialsInfo,
				accessToken: try sqlRow["accessToken"]!.asString(),
				databaseKey: databaseKey,
				encryptedPassword: try sqlRow["encryptedPassword"]!.asString()
			)
		}
	}

	func store(credentials: PersistedCredentials) throws {
		let databaseKey: TaggedSqlValue = if let databaseKey = credentials.databaseKey { .string(value: databaseKey) } else { .null }
		try db.prepare(
			query: """
				INSERT INTO credentials (login, userId, type, accessToken, databaseKey, encryptedPassword) 
				VALUES (?, ?, ?, ?, ?, ?)
				"""
		)
		.bindParams([
			.string(value: credentials.credentialInfo.login), .string(value: credentials.credentialInfo.userId),
			.string(value: credentials.credentialInfo.type.rawValue), .string(value: credentials.accessToken), databaseKey,
			.string(value: credentials.encryptedPassword),
		])
		.run()
	}
}

private extension TaggedSqlValue {
	struct InvalidSqlType: Error { init() {} }

	func asString() throws -> String { if case let .string(value) = self { return value } else { throw InvalidSqlType() } }
}
