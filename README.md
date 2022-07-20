# libtrash

[![](https://img.shields.io/github/v/tag/thechampagne/libtrash?label=version)](https://github.com/thechampagne/libtrash/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libtrash)](https://github.com/thechampagne/libtrash/blob/main/LICENSE)

A **C** library for moving files and folders to the Recycle Bin.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libtrash.git
```
#### 2. Navigate to the root
```
cd libtrash
```
#### 3. Build the project
```
cargo build
```

### Available functions

```c
#include <stdio.h>
#include <trash.h>

int main()
{
  const char* arr[] = { "file.txt", "file2.txt" };
  if (trash_delete_all(arr, 2) != 0)
  {
    printf("Something went wrong with trash_delete_all\n");
    return -1;
  }

  if (trash_delete("file.txt") != 0)
  {
    printf("Something went wrong with trash_delete\n");
    return -1;
  }
  return 0;
}
```

### References
 - [trash](https://github.com/Byron/trash-rs)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libtrash/blob/main/LICENSE).
