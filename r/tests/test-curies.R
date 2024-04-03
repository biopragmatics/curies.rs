library(testthat)
library(curiesr)

test_that("Create curiesr default converter, compress and expand", {
  converter <- ConverterR$new()
  expect_equal(converter$compress("http://purl.obolibrary.org/obo/DOID_1234"), "doid:1234")
  expect_equal(converter$expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234")

  # curie <- converter$compress("http://purl.obolibrary.org/obo/DOID_1234")
  # print(curie)
})
