"""Small semantic checks for delta accounting and mixed Rust provenance."""
import unittest
import refresh

class DeltaChecks(unittest.TestCase):
 def test_inherited_formatting_and_movement_are_not_authorship(self):
  self.assertEqual(refresh.delta('alpha\nbeta\n','  beta\n alpha \n'),(set(),set()))
 def test_replacement_is_one_new_line_and_one_deleted_line(self):
  self.assertEqual(refresh.delta('same\nold\ntail\n','same\nnew\ntail\n'),({1},{1}))
 def test_deleted_only_is_not_retained_effort(self):
  self.assertEqual(refresh.delta('same\nremoved\n','same\n'),(set(),{1}))
 def test_initial_supplied_content_is_not_new_requirements_work(self):
  self.assertEqual(refresh.delta('HLR retained\n','HLR retained\nLLR added\n'),({1},set()))
 def test_named_test_module_and_spec_function(self):
  text='pub open spec fn p() -> bool {\n true\n}\n#[cfg(test)]\nmod policy_tests {\n fn case() { let message = "}"; }\n}\nfn production() {}\n'
  tests,proofs=refresh.rust_regions(text)
  self.assertEqual(tests,{3,4,5,6})
  self.assertEqual(proofs,{0,1,2})
 def test_gumbo_trailing_blank_preserves_line_coordinates(self):
  text='part def A {\n language "GUMBO" /*{\n guarantee test: true;\n\n }*/\n}\n'
  classified=refresh.classes(text,'sysml')
  self.assertEqual(len(classified),6)
  self.assertEqual(classified[2:4],['code','blank'])

if __name__=='__main__':unittest.main()
