(function() {var implementors = {};
implementors["halo2"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"halo2/circuit/struct.RegionIndex.html\" title=\"struct halo2::circuit::RegionIndex\">RegionIndex</a>","synthetic":false,"types":["halo2::circuit::RegionIndex"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"halo2/circuit/struct.RegionStart.html\" title=\"struct halo2::circuit::RegionStart\">RegionStart</a>","synthetic":false,"types":["halo2::circuit::RegionStart"]},{"text":"impl&lt;'r, F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'r mut (dyn <a class=\"trait\" href=\"halo2/circuit/layouter/trait.RegionLayouter.html\" title=\"trait halo2::circuit::layouter::RegionLayouter\">RegionLayouter</a>&lt;F&gt; + 'r)&gt; for <a class=\"struct\" href=\"halo2/circuit/struct.Region.html\" title=\"struct halo2::circuit::Region\">Region</a>&lt;'r, F&gt;","synthetic":false,"types":["halo2::circuit::Region"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Advice.html\" title=\"struct halo2::plonk::Advice\">Advice</a>&gt; for <a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>","synthetic":false,"types":["halo2::plonk::circuit::Any"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Fixed.html\" title=\"struct halo2::plonk::Fixed\">Fixed</a>&gt; for <a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>","synthetic":false,"types":["halo2::plonk::circuit::Any"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Instance.html\" title=\"struct halo2::plonk::Instance\">Instance</a>&gt; for <a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>","synthetic":false,"types":["halo2::plonk::circuit::Any"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Advice.html\" title=\"struct halo2::plonk::Advice\">Advice</a>&gt;&gt; for <a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>&gt;","synthetic":false,"types":["halo2::plonk::circuit::Column"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Fixed.html\" title=\"struct halo2::plonk::Fixed\">Fixed</a>&gt;&gt; for <a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>&gt;","synthetic":false,"types":["halo2::plonk::circuit::Column"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Instance.html\" title=\"struct halo2::plonk::Instance\">Instance</a>&gt;&gt; for <a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>&gt;","synthetic":false,"types":["halo2::plonk::circuit::Column"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;F&gt; for <a class=\"enum\" href=\"halo2/plonk/enum.Assigned.html\" title=\"enum halo2::plonk::Assigned\">Assigned</a>&lt;F&gt;","synthetic":false,"types":["halo2::plonk::circuit::Assigned"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>F, F<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"enum\" href=\"halo2/plonk/enum.Assigned.html\" title=\"enum halo2::plonk::Assigned\">Assigned</a>&lt;F&gt;","synthetic":false,"types":["halo2::plonk::circuit::Assigned"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Expression.html\" title=\"enum halo2::plonk::Expression\">Expression</a>&lt;F&gt;&gt; for <a class=\"struct\" href=\"halo2/plonk/struct.Constraint.html\" title=\"struct halo2::plonk::Constraint\">Constraint</a>&lt;F&gt;","synthetic":false,"types":["halo2::plonk::circuit::Constraint"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>&amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"enum\" href=\"halo2/plonk/enum.Expression.html\" title=\"enum halo2::plonk::Expression\">Expression</a>&lt;F&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"halo2/plonk/struct.Constraint.html\" title=\"struct halo2::plonk::Constraint\">Constraint</a>&lt;F&gt;","synthetic":false,"types":["halo2::plonk::circuit::Constraint"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"halo2/arithmetic/trait.Field.html\" title=\"trait halo2::arithmetic::Field\">Field</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Expression.html\" title=\"enum halo2::plonk::Expression\">Expression</a>&lt;F&gt;&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Constraint.html\" title=\"struct halo2::plonk::Constraint\">Constraint</a>&lt;F&gt;&gt;","synthetic":false,"types":["alloc::vec::Vec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"halo2/dev/metadata/struct.Column.html\" title=\"struct halo2::dev::metadata::Column\">Column</a>","synthetic":false,"types":["halo2::dev::metadata::Column"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"halo2/plonk/struct.Column.html\" title=\"struct halo2::plonk::Column\">Column</a>&lt;<a class=\"enum\" href=\"halo2/plonk/enum.Any.html\" title=\"enum halo2::plonk::Any\">Any</a>&gt;&gt; for <a class=\"struct\" href=\"halo2/dev/metadata/struct.Column.html\" title=\"struct halo2::dev::metadata::Column\">Column</a>","synthetic":false,"types":["halo2::dev::metadata::Column"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"halo2/dev/metadata/struct.Gate.html\" title=\"struct halo2::dev::metadata::Gate\">Gate</a>","synthetic":false,"types":["halo2::dev::metadata::Gate"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"struct\" href=\"halo2/dev/metadata/struct.Gate.html\" title=\"struct halo2::dev::metadata::Gate\">Gate</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"halo2/dev/metadata/struct.Constraint.html\" title=\"struct halo2::dev::metadata::Constraint\">Constraint</a>","synthetic":false,"types":["halo2::dev::metadata::Constraint"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"halo2/dev/metadata/struct.Region.html\" title=\"struct halo2::dev::metadata::Region\">Region</a>","synthetic":false,"types":["halo2::dev::metadata::Region"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()