import radix_wallet_kit

public typealias SUT = [FactorSource]
extension SUT {
	public static let placeholder: Self = newFactorSourcesPlaceholder()
	public static let placeholderOther: Self = newFactorSourcesPlaceholderOther()
}

func test_equatable() throws {
	let a = SUT.placeholder
	let b = SUT.placeholderOther
	assert(
		a == SUT.placeholder
	)
	assert(a != b)
	assert(b == SUT.placeholderOther)
}

func test_hashable() throws {
	let a = SUT.placeholder
	let b = SUT.placeholderOther
	assert(Set([a, a]).count == 1)
	assert(Set([b, b]).count == 1)
	assert(Set([a, b, b, a]).count == 2)
}

func test() throws {
	try test_equatable()
	try test_hashable()
}

try! test()
