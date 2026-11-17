import unittest

from check import diagnostics, folder_sizes
from rust import Size, measure


class RustSizeTests(unittest.TestCase):
    def test_inline_test_module_is_excluded_from_production_size(self):
        source = "fn production() {}\n#[cfg(test)]\nmod tests {\n fn test() {}\n}\n"
        self.assertEqual(measure("src/lib.rs", source), Size(source=1, tests=4))

    def test_rust_literals_and_comments_do_not_confuse_test_module_counting(self):
        source = '''fn production() {}
#[cfg(test)]
mod tests {
    const RAW: &str = r###" } #[cfg(test)] { "###;
    /* } /* nested { */ */
    fn test() {}
}
'''
        self.assertEqual(measure("src/lib.rs", source), Size(source=1, tests=6))


class StructureGuardrailTests(unittest.TestCase):
    def test_new_source_over_limit_fails(self):
        self.assertEqual(
            diagnostics({"src/large.rs": Size(801, 0)}),
            [("error", "src/large.rs", "801 non-test lines; limit 800")],
        )

    def test_existing_source_must_shrink_and_hard_limit_always_fails(self):
        baseline = {"src/legacy.rs": Size(900, 0)}
        self.assertEqual(
            diagnostics({"src/legacy.rs": Size(850, 0)}, baseline, {"src/legacy.rs"})[0][0],
            "warning",
        )
        self.assertEqual(
            diagnostics({"src/legacy.rs": Size(901, 0)}, baseline, {"src/legacy.rs"})[0][0],
            "error",
        )
        self.assertEqual(
            diagnostics({"src/legacy.rs": Size(1501, 0)}, baseline, set())[0][0],
            "error",
        )

    def test_test_files_have_a_hard_limit(self):
        rows = diagnostics({"tests/large.rs": Size(0, 1001)})
        self.assertEqual(rows[0][0], "error")

    def test_module_families_and_folder_breadth_are_detected(self):
        paths = ["src/session.rs", "src/session_events.rs", "src/session_tools.rs"]
        self.assertEqual(folder_sizes(paths), {("family", "src/session_*"): 2})
        paths = [f"src/module{i}.rs" for i in range(61)]
        self.assertEqual(folder_sizes(paths), {("folder", "src"): 61})


if __name__ == "__main__":
    unittest.main()
