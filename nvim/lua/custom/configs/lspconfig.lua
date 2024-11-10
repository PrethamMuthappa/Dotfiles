local on_attach = require("plugins.configs.lspconfig").on_attach
local capabilities = require("plugins.configs.lspconfig").capabilities

local lspconfig = require("lspconfig")

lspconfig.rust_analyzer.setup({
  on_attach = on_attach,
  capabilities = capabilities,
  filetypes = {"rust"},
  -- Modify the root_dir logic to look both up and down the directory tree
  root_dir = function(fname)
    local util = require('lspconfig.util')
    return util.root_pattern('Cargo.toml', 'rust-project.json')(fname) or
           util.find_git_ancestor(fname) or
           util.path.dirname(fname)
  end,
  settings = {
    ['rust-analyzer'] = {
      cargo = {
        allFeatures = true,
      },
      -- Add these settings to help with project detection
      procMacro = {
        enable = true
      },
      checkOnSave = {
        command = "check"
      },
    },
  },
})
