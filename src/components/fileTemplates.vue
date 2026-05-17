<script>
const fileTemplates = {
  txt: {
    allowSeparateType: false,
    variants: {
      plain: {
        label: 'Plain text',
        extensions: ['.txt'],
        template: '',
      },
    },
  },
  md: {
    allowSeparateType: false,
    variants: {
      article: {
        label: 'Markdown',
        extensions: ['.md'],
        template: `# Title

Write your text here.
`,
      },
    },
  },
  rust: {
    allowSeparateType: false,
    variants: {
      binary: {
        label: 'Binary',
        extensions: ['.rs'],
        template:
            `fn main() {
    println!("Hello, Rust!");
}
`,
      },
    },
  },
  python: {
    allowSeparateType: false,
    variants: {
      file: {
        label: 'Python File',
        extensions: ['.py'],
        template: `def main():
    print("Hello, Python!")


if __name__ == "__main__":
    main()
`,
      },
      unitTest: {
        label: 'Python unit test',
        extensions: ['.py'],
        template: `import unittest


class TestExample(unittest.TestCase):
    def test_something(self):
        self.assertTrue(True)


if __name__ == "__main__":
    unittest.main()
`,
      },
    },
  },
  lua: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'Lua script',
        extensions: ['.lua'],
        template: `print("Hello, Lua!")
`,
      },
    },
  },
  gamma: {
    allowSeparateType: true,
    variants: {
      script: {
        label: 'Gamma script',
        extensions: ['.gamma', '.gma', '.gm'],
        template: `# Gamma file
`,
      },
    },
  },
  html: {
    allowSeparateType: false,
    variants: {
      page: {
        label: 'HTML page',
        extensions: ['.html'],
        template: `<!doctype html>
                  <html lang="en">
                  <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>Document</title>
                  </head>
                  <body>

                  </body>
                  </html>
                  `,
      },
    },
  },
  css: {
    allowSeparateType: false,
    variants: {
      stylesheet: {
        label: 'CSS stylesheet',
        extensions: ['.css'],
        template: `:root {
                    --primary: #4f46e5;
                  }

                  * {
                    box-sizing: border-box;
                  }

                  body {
                    margin: 0;
                    font-family: sans-serif;
                  }
                  `,
      },
    },
  },
  js: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'JavaScript',
        extensions: ['.js', '.jsx'],
        template: `console.log("Hello, JavaScript!");
`,
      },
      component: {
        label: 'React JSX',
        extensions: ['.jsx'],
        template: `export default function App() {
  return <div>Hello, React</div>;
}
`,
      },
    },
  },
  ts: {
    allowSeparateType: false,
    variants: {
      script: {
        label: 'TypeScript',
        extensions: ['.ts', '.tsx'],
        template: `const message: string = "Hello, TypeScript!";
console.log(message);
`,
      },

      component: {
        label: 'React TSX',
        extensions: ['.tsx'],
        template: `export default function App(): JSX.Element {
  return <div>Hello, TypeScript React</div>;
}
`,
      },
    },
  },
  vue: {
    allowSeparateType: false,
    variants: {

    },
  },
  ccpp: {
    allowSeparateType: false,
    variants: {
      c: {
        label: 'C File',
        extensions: ['.c'],
        template: `#include <stdio.h>

int main(void) {
    printf("Hello, C!\\n");
    return 0;
}
`,
      },

      cpp: {
        label: 'C++ File',
        extensions: ['.cpp'],
        template: `#include <iostream>

int main() {
    std::cout << "Hello, C++!" << std::endl;
    return 0;
}
`,
      },

      h: {
        label: 'C Header',
        extensions: ['.h'],
        template: `#pragma once
`,
      },

      hpp: {
        label: 'C++ Header',
        extensions: ['.hpp'],
        template: `#pragma once
`,
      },
    }
  },
  asm: {
    allowSeparateType: true,
    variants: {
      nasm: {
        label: 'NASM',
        template: `; NASM template
`,
      },

      fasm: {
        label: 'FASM',
        template: `; FASM template
`,
      },

      tasm: {
        label: 'TASM',
        template: `; TASM template
`,
      },

      rasm: {
        label: 'RASM',
        template: `arch x86;
format elf64;

declare number dw = 84;
declare msg1 char[] = "Grether than 150";
declare msg2 char[] = "Less than 150";

entry _start;
# simple comment

_start:
    mov rax, 84;
    add rax, number;
    if(rax >= 150){
        print msg1;
    } else {
        print msg2;
    }
    print rax;
`,
      },
    }
  },
};

export default fileTemplates;
</script>