#ifndef __TRASH_H__
#define __TRASH_H__

#ifdef __cplusplus
extern "C" {
#endif

/*
* Delete a file.
* 
* Example:
* * *
* int main()
* {
*   if (trash_delete("file.txt") != 0)
*   {
*     printf("Something went wrong\n");
*     return -1;
*   }
*   return 0;
* }
* * *
* 
* @param path
* @return 0 on success and non zero value on failure
*/
extern int trash_delete(const char* path);

/*
* Delete multiple files.
* 
* Example:
* * *
* int main()
* {
*   const char* arr[] = { "file.txt", "file2.txt" };
*   if (trash_delete_all(arr, 2) != 0)
*   {
*     printf("Something went wrong\n");
*     return -1;
*   }
*   return 0;
* }
* * *
* 
* @param path
* @param length
* @return 0 on success and non zero value on failure
*/
extern int trash_delete_all(const char** paths, size_t length);

#ifdef __cplusplus
}
#endif

#endif // __TRASH_H__